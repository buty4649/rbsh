mod option;
mod redirect;

pub mod syscall;
pub use syscall::SysCallError;

use super::{
    context::Context,
    error::ShellErrorKind,
    location::Location,
    parser::{
        redirect::{Redirect, RedirectList},
        word::{Word, WordKind, WordList},
        ConnecterKind, Unit, UnitKind,
    },
    signal::{restore_tty_signals, JobSignalHandler},
    status::ExitStatus,
};
use nix::{
    errno::Errno,
    unistd::{ForkResult, Pid},
};
use option::{ExecOption, ExecOptionBuilder};
use redirect::ApplyRedirect;
use syscall::{SysCallResult, SysCallWrapper, Wrapper};

use is_executable::IsExecutable;
use std::{collections::HashMap, env, ffi::CString, os::unix::io::RawFd, path::PathBuf};

pub trait WordParser {
    fn to_string<'a>(self, context: &Context) -> String;
}

impl WordParser for Word {
    fn to_string(self, context: &Context) -> String {
        let (s, k, _) = self.take();
        match k {
            WordKind::Normal | WordKind::Quote | WordKind::Literal => s,
            WordKind::Command => "".to_string(), // unimplemented
            WordKind::Variable | WordKind::Parameter => {
                context.get_var(s).unwrap_or("".to_string())
            }
        }
    }
}

impl WordParser for WordList {
    fn to_string(self, context: &Context) -> String {
        self.to_vec()
            .into_iter()
            .fold(String::new(), |mut result, word| {
                result.push_str(&*word.to_string(context));
                result
            })
    }
}

pub trait IsPresent {
    fn is_present(&self) -> bool;
}
impl<T> IsPresent for Vec<T> {
    fn is_present(&self) -> bool {
        !self.is_empty()
    }
}
impl<T, U> IsPresent for HashMap<T, U> {
    fn is_present(&self) -> bool {
        !self.is_empty()
    }
}

pub trait ToCString<T> {
    fn to_cstring(self) -> T;
}

impl ToCString<CString> for &str {
    fn to_cstring(self) -> CString {
        CString::new(self).unwrap()
    }
}

impl ToCString<Vec<CString>> for Vec<String> {
    fn to_cstring(self) -> Vec<CString> {
        self.into_iter().map(|s| s.to_cstring()).collect::<Vec<_>>()
    }
}

pub trait IsVarName {
    fn is_var_name(&self) -> bool;
}

impl IsVarName for WordList {
    fn is_var_name(&self) -> bool {
        match self.first().take() {
            (string, WordKind::Normal, _) => {
                let mut c = string.chars();

                // first char is must alphanumeric
                match c.next() {
                    Some(c) if c.is_alphanumeric() => true,
                    _ => return false,
                };

                loop {
                    match c.next() {
                        Some(c) if c == '=' => break true,
                        Some(c) if c.is_alphanumeric() || c == '_' => continue,
                        _ => break false,
                    }
                }
            }
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Job {
    id: u16,
    pgid: Pid,
    processes: Vec<Process>,
}

impl Job {
    fn new(id: u16, pgid: Pid) -> Self {
        Self {
            id,
            pgid,
            processes: vec![],
        }
    }

    fn append(&mut self, process: Process) {
        self.processes.push(process)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Process {
    pid: Pid,
    status: ProcessStatus,
}

#[derive(Debug, Copy, Clone)]
pub enum ProcessStatus {
    Running,
    Exited(ExitStatus),
}

impl Process {
    pub fn running(pid: Pid) -> Self {
        Self {
            pid,
            status: ProcessStatus::Running,
        }
    }

    pub fn exit(mut self, status: ExitStatus) {
        self.status = ProcessStatus::Exited(status)
    }
}

struct ChildProcess {
    pid: Pid,
    pgid: Pid,
    pipe: RawFd,
}

impl ChildProcess {
    fn new(pid: Pid, pgid: Pid, pipe: RawFd) -> Self {
        Self { pid, pgid, pipe }
    }

    fn pid(&self) -> Pid {
        self.pid
    }

    fn pgid(&self) -> Pid {
        self.pgid
    }

    fn start(&self, wrapper: &Wrapper) {
        wrapper.close(self.pipe).ok();
    }
}

pub struct Executor {
    ctx: Context,
    handler: JobSignalHandler,
    pub job_id: u16,
    jobs: Vec<Job>,
}

impl Executor {
    pub fn new(wrapper: Wrapper) -> std::result::Result<Self, std::io::Error> {
        Ok(Self {
            ctx: Context::new(wrapper),
            handler: JobSignalHandler::start()?,
            job_id: 0,
            jobs: vec![],
        })
    }

    pub fn context(&self) -> &Context {
        &self.ctx
    }

    fn wrapper(&self) -> &Wrapper {
        &self.ctx.wrapper()
    }

    pub fn execute_command(&mut self, cmd: Unit, option: Option<ExecOption>) -> ExitStatus {
        let ret = match cmd.kind() {
            UnitKind::SimpleCommand { command, redirect } => {
                let opt = ExecOptionBuilder::from(option)
                    .if_then(cmd.background(), |b| b.background())
                    .build();
                let ret = self.execute_simple_command(command, redirect, Some(opt));
                if cmd.background() {
                    let job = self.jobs.last().unwrap();
                    eprintln!("[{}] {}", job.id, job.pgid);
                }
                ret
            }
            kind => {
                let background = cmd.background()
                    || match kind {
                        UnitKind::Connecter {
                            left: _,
                            right: _,
                            kind: _,
                        } => false,
                        _ => option.and_then(|o| Some(o.background())).unwrap_or(false),
                    };
                let opt = if !background {
                    option
                } else {
                    let pgid = option.and_then(|o| o.pgid());
                    match self.fork(pgid) {
                        Err(e) => {
                            eprintln!("{}: {}", e.name(), e.desc());
                            return ExitStatus::failure();
                        }
                        Ok(Some(child)) => {
                            child.start(self.wrapper());
                            let ret = self.start_job(child.pid(), child.pgid());
                            if cmd.background() {
                                let job = self.jobs.last().unwrap();
                                eprintln!("[{}] {}", job.id, job.pgid);
                            }
                            return ret;
                        }
                        Ok(None) => {
                            self.handler = JobSignalHandler::start().unwrap();
                            let pgid = pgid.unwrap_or(self.wrapper().getpid());
                            let opt = ExecOptionBuilder::from(option)
                                .foreground()
                                .pgid(pgid)
                                .build();
                            Some(opt)
                        }
                    }
                };
                let ret = match kind {
                    UnitKind::SimpleCommand {
                        command: _,
                        redirect: _,
                    } => unreachable![],
                    UnitKind::Connecter { left, right, kind } => {
                        self.execute_connecter(left, right, kind, opt)
                    }
                    UnitKind::If {
                        condition,
                        true_case,
                        false_case,
                        redirect,
                    } => self
                        .execute_if_command(condition, true_case, false_case, redirect, false, opt),
                    UnitKind::Unless {
                        condition,
                        false_case,
                        true_case,
                        redirect,
                    } => self
                        .execute_if_command(condition, false_case, true_case, redirect, true, opt),
                    UnitKind::While {
                        condition,
                        command,
                        redirect,
                    } => self.execute_while_command(condition, command, redirect, false, opt),
                    UnitKind::Until {
                        condition,
                        command,
                        redirect,
                    } => self.execute_while_command(condition, command, redirect, true, opt),
                    UnitKind::For {
                        identifier,
                        list,
                        command,
                        redirect,
                    } => self.execute_for_command(identifier, list, command, redirect, opt),
                };

                match background {
                    false => ret,
                    true => self.wrapper().exit(ret.code()),
                }
            }
        };

        ret
    }

    fn execute_simple_command(
        &mut self,
        command: Vec<WordList>,
        mut redirect: RedirectList,
        option: Option<ExecOption>,
    ) -> ExitStatus {
        let (temp_env, cmds) = split_env_and_commands(&self.ctx, command);
        if let Some(pipe) = option.and_then(|o| o.input()) {
            redirect.push(Redirect::copy(pipe, 0, true, Location::new(0, 0)));
        }
        if let Some((pipe, both)) = option.and_then(|o| o.output()) {
            redirect.push(Redirect::copy(pipe, 1, true, Location::new(0, 0)));
            if both {
                redirect.push(Redirect::copy(1, 2, false, Location::new(0, 0)));
            }
        }

        if cmds.is_empty() && temp_env.is_present() {
            temp_env.iter().for_each(|(k, v)| self.ctx.set_var(k, v));
            ExitStatus::success()
        } else if cmds.is_present() {
            let pgid = option.and_then(|o| o.pgid());
            match self.fork(pgid) {
                Err(e) => {
                    eprintln!("{}: {}", e.name(), e.desc());
                    ExitStatus::failure()
                }
                Ok(Some(child)) => {
                    let background = option.and_then(|o| Some(o.background())).unwrap_or(false);
                    if background {
                        child.start(self.wrapper());
                        self.start_job(child.pid(), pgid.unwrap_or(child.pgid))
                    } else {
                        let old_pgrp =
                            if pgid.is_none() && self.wrapper().isatty(0).unwrap_or(false) {
                                let old = self.wrapper().tcgetpgrp(0).ok();
                                self.wrapper().tcsetpgrp(0, child.pid()).ok();
                                old
                            } else {
                                None
                            };

                        self.handler.set_forground(child.pgid());
                        child.start(self.wrapper());
                        self.start_job(child.pid(), child.pgid());
                        let ret = self
                            .handler
                            .wait_for(child.pid(), true)
                            .unwrap_or(ExitStatus::failure());
                        self.handler.reset_forground();

                        self.jobs.pop(); // remove current job

                        if let Some(p) = old_pgrp {
                            self.wrapper().tcsetpgrp(0, p).ok();
                        }

                        ret
                    }
                }
                Ok(None) => {
                    let cmdpath = cmds.first().unwrap().to_string();
                    execute_external_command(&self.ctx, cmdpath, cmds, temp_env, redirect)
                }
            }
        } else {
            // noop
            ExitStatus::success()
        }
    }

    fn execute_connecter(
        &mut self,
        left: Box<Unit>,
        right: Box<Unit>,
        kind: ConnecterKind,
        option: Option<ExecOption>,
    ) -> ExitStatus {
        match kind {
            ConnecterKind::And => match self.execute_command(*left, option) {
                status if status.is_error() => status,
                _ => self.execute_command(*right, option),
            },
            ConnecterKind::Or => match self.execute_command(*left, option) {
                status if status.is_success() => status,
                _ => self.execute_command(*right, option),
            },
            ConnecterKind::Pipe | ConnecterKind::PipeBoth => {
                let (pipe_read, pipe_write) = pipe(self.wrapper()).unwrap();

                let opt = ExecOptionBuilder::from(option)
                    .background()
                    .output(pipe_write)
                    .if_then(kind == ConnecterKind::PipeBoth, |b| b.both_output())
                    .build();
                self.execute_command(*left, Some(opt));

                let opt = ExecOptionBuilder::from(option)
                    .background()
                    .default_pgid(self.jobs.last().unwrap().pgid)
                    .input(pipe_read)
                    .build();
                self.execute_command(*right, Some(opt));

                self.wrapper().close(pipe_read).unwrap();
                self.wrapper().close(pipe_write).unwrap();

                let background = option.and_then(|o| Some(o.background())).unwrap_or(false);
                if background {
                    ExitStatus::success()
                } else {
                    let job = self.jobs.pop().unwrap();
                    self.handler.set_forground(job.pgid);
                    let statuses = job
                        .processes
                        .iter()
                        .map(|process| {
                            self.handler
                                .wait_for(process.pid, true)
                                .unwrap_or(ExitStatus::failure())
                        })
                        .collect::<Vec<_>>();
                    self.handler.reset_forground();

                    statuses.last().unwrap_or(&ExitStatus::success()).to_owned()
                }
            }
        }
    }

    fn execute_if_command(
        &mut self,
        condition: Box<Unit>,
        true_case: Vec<Unit>,
        false_case: Option<Vec<Unit>>,
        _redirect: RedirectList,
        inverse: bool,
        option: Option<ExecOption>,
    ) -> ExitStatus {
        match self.execute_command(*condition, option) {
            status if (!inverse && status.is_success()) || (inverse && status.is_error()) => {
                let s = true_case
                    .into_iter()
                    .map(|command| self.execute_command(command, option));
                s.last().unwrap()
            }
            status if false_case.is_none() => status,
            _ => {
                let s = false_case
                    .unwrap()
                    .into_iter()
                    .map(|command| self.execute_command(command, option));
                s.last().unwrap()
            }
        }
    }

    fn execute_while_command(
        &mut self,
        condition: Box<Unit>,
        command: Vec<Unit>,
        _redirect: RedirectList,
        inverse: bool,
        option: Option<ExecOption>,
    ) -> ExitStatus {
        'exec: loop {
            macro_rules! interrupt {
                () => {
                    if self.handler.is_interrupt() {
                        break 'exec;
                    }
                };
            }

            interrupt!();
            match self.execute_command(*condition.clone(), option) {
                status if (!inverse && status.is_success()) || (inverse && status.is_error()) => {
                    for c in command.to_vec() {
                        interrupt!();
                        self.execute_command(c, option);
                    }
                }
                _ => break 'exec,
            }
        }
        ExitStatus::new(0)
    }

    fn execute_for_command(
        &mut self,
        identifier: Word,
        list: Option<Vec<WordList>>,
        command: Vec<Unit>,
        _redirect: RedirectList,
        option: Option<ExecOption>,
    ) -> ExitStatus {
        let identifier = match identifier.take() {
            (string, kind, _) if kind == WordKind::Normal => string.to_string(),
            (string, kind, _) => {
                let _ = match kind {
                    WordKind::Normal => unreachable![],
                    WordKind::Quote => format!("\"{}\"", string),
                    WordKind::Literal => format!("'{}'", string),
                    WordKind::Command => format!("`{}`", string),
                    WordKind::Variable => format!("${}", string),
                    WordKind::Parameter => format!("${{{}}}", string),
                };
                eprintln!("error: invalid identifier");
                return ExitStatus::failure();
            }
        };

        let list = match list {
            None => vec![], // Normally, it returns $@.
            Some(list) => list
                .into_iter()
                .map(|w| w.to_string(&self.ctx))
                .collect::<Vec<_>>(),
        };

        'exec: for word in list.iter() {
            self.ctx.set_var(&*identifier, word);
            for c in command.to_vec() {
                if self.handler.is_interrupt() {
                    break 'exec;
                }

                self.execute_command(c, option);
            }
        }

        ExitStatus::new(0)
    }

    fn fork(
        &mut self,
        pgid: Option<Pid>,
    ) -> std::result::Result<Option<ChildProcess>, SysCallError> {
        let (tmp_read, tmp_write) = pipe(self.wrapper())?;
        match self.wrapper().fork() {
            Err(e) => {
                self.wrapper().close(tmp_read).ok();
                self.wrapper().close(tmp_write).ok();
                Err(e)
            }
            Ok(ForkResult::Parent { child }) => {
                let new_pgid = pgid.unwrap_or(child);
                self.wrapper().setpgid(child, new_pgid).ok();
                self.wrapper().close(tmp_read).ok();

                let ret = ChildProcess::new(child, new_pgid, tmp_write);
                Ok(Some(ret))
            }
            Ok(ForkResult::Child) => {
                self.wrapper().close(tmp_write).ok();

                // Synchronize with the parent process.
                loop {
                    let mut buf = [0];
                    match self.wrapper().read(tmp_read, &mut buf) {
                        // Read again because it was interrupted by Signal.
                        Err(e) if e.errno() == nix::errno::Errno::EINTR => (),
                        _ => break,
                    }
                }
                self.wrapper().close(tmp_read).ok();

                Ok(None)
            }
        }
    }

    fn start_job(&mut self, pid: Pid, pgid: Pid) -> ExitStatus {
        let is_new_job = match self.jobs.last() {
            None => true,
            Some(job) => job.pgid != pgid,
        };

        if is_new_job {
            self.job_id += 1;
            let mut job = Job::new(self.job_id, pid);
            job.append(Process::running(pid));
            self.jobs.push(job);
        } else {
            let mut job = self.jobs.pop().unwrap();
            job.processes.push(Process::running(pid));
            self.jobs.push(job);
        }

        ExitStatus::success()
    }

    pub fn reap_job(&mut self) {
        self.jobs = self
            .jobs
            .iter()
            .filter(|job| {
                let mut exited = 0;
                for process in job.processes.iter() {
                    match process.status {
                        ProcessStatus::Running => match self.handler.wait_for(process.pid, false) {
                            None => (),
                            Some(status) => {
                                exited += 1;
                                process.exit(status);
                            }
                        },
                        ProcessStatus::Exited(_) => exited += 1,
                    }
                }

                if exited == job.processes.len() {
                    eprintln!("[{}]+ Done", job.id);
                    false
                } else {
                    // remove job
                    true
                }
            })
            .map(|job| job.clone())
            .collect::<Vec<_>>();
        self.jobs.sort_by(|a, b| a.id.cmp(&b.id));
        self.job_id = self.jobs.last().map(|job| job.id).unwrap_or(0);
    }
}

fn split_env_and_commands(
    ctx: &Context,
    list: Vec<WordList>,
) -> (HashMap<String, String>, Vec<String>) {
    let (env, cmds) = {
        let mut env = vec![];
        let mut cmds = vec![];
        let mut iter = list.into_iter().peekable();

        loop {
            match iter.peek() {
                Some(wl) if wl.is_var_name() => {
                    let wl = iter.next().unwrap();
                    env.push(wl.clone())
                }
                _ => break,
            }
        }

        loop {
            match iter.next() {
                Some(wl) => cmds.push(wl.clone()),
                None => break,
            }
        }

        (env, cmds)
    };

    let env = env
        .into_iter()
        .map(|wordlist| {
            wordlist
                .to_string(ctx)
                .split_once("=")
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .unwrap()
        })
        .collect::<HashMap<_, _>>();
    let cmds = cmds
        .into_iter()
        .map(|wordlist| wordlist.to_string(ctx))
        .collect::<Vec<_>>();
    (env, cmds)
}

fn assume_command(command: &str) -> PathBuf {
    let mut buf = PathBuf::new();
    buf.push(command);

    if buf.is_absolute() || buf.starts_with(".") {
        buf
    } else {
        // search command
        match env::var("PATH") {
            Err(_) => buf,
            Ok(val) => val
                .split(":")
                .find_map(|p| {
                    let mut buf = PathBuf::new();
                    buf.push(p);
                    buf.push(command);
                    if buf.is_file() && buf.is_executable() {
                        Some(buf)
                    } else {
                        None
                    }
                })
                .unwrap_or(buf),
        }
    }
}

fn execute_external_command<T: AsRef<str>>(
    ctx: &Context,
    path: T,
    cmds: Vec<String>,
    temp_env: HashMap<String, String>,
    redirect: RedirectList,
) -> ExitStatus {
    let path = path.as_ref();
    let cmdpath = assume_command(path).to_str().unwrap().to_cstring();
    let cmds = cmds.to_cstring();

    // merge temporary env to env
    let mut env = env::vars().collect::<HashMap<_, _>>();
    temp_env.into_iter().for_each(|(k, v)| {
        env.insert(k, v);
    });
    let env = env
        .into_iter()
        .map(|(k, v)| format!("{}={}", k, v).to_cstring())
        .collect::<Vec<_>>();

    if let Err(e) = redirect.apply(ctx) {
        match e.value() {
            ShellErrorKind::SysCallError(f, e) => {
                eprintln!("{}: {}", f, e.desc());
                return ctx.wrapper().exit(1);
            }
            _ => unreachable![],
        }
    }

    restore_tty_signals(ctx.wrapper()).unwrap();

    match ctx.wrapper().execve(cmdpath, &cmds, &env) {
        Ok(_) => unreachable![],
        Err(e) if e.errno() == Errno::ENOENT => {
            eprintln!("{}: command not found", path);
            ctx.wrapper().exit(127)
        }
        Err(e) => {
            eprintln!("execve faile: {}", e.errno());
            ctx.wrapper().exit(1)
        }
    }
}

fn pipe(wrapper: &Wrapper) -> SysCallResult<(RawFd, RawFd)> {
    let temp = wrapper.pipe()?;
    let read = wrapper.dup_fd(temp.0, 255).unwrap();
    let write = wrapper.dup_fd(temp.1, 255).unwrap();
    wrapper.close(temp.0).ok();
    wrapper.close(temp.1).ok();
    Ok((read, write))
}

include!("exec_test.rs");
