mod redirect;

pub mod syscall;
pub use syscall::SysCallError;

use super::{
    context::Context,
    error::ShellErrorKind,
    parser::{
        redirect::RedirectList,
        word::{Word, WordKind, WordList},
        ConnecterKind, UnitKind,
    },
    signal::{restore_tty_signals, JobSignalHandler},
    status::ExitStatus,
};
use nix::{
    errno::Errno,
    unistd::{ForkResult, Pid},
};
use redirect::ApplyRedirect;
use syscall::{SysCallWrapper, Wrapper};

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
    pipe: RawFd,
}

impl ChildProcess {
    fn new(pid: Pid, pipe: RawFd) -> Self {
        Self { pid, pipe }
    }

    fn pid(&self) -> Pid {
        self.pid
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

    pub fn execute_command(&mut self, cmd: UnitKind) -> ExitStatus {
        let ret = self.execute_command_internal(cmd, None, None, None);

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

        ret
    }

    pub fn execute_command_internal(
        &mut self,
        cmd: UnitKind,
        pgid: Option<Pid>,
        pipe_read: Option<RawFd>,
        pipe_write: Option<RawFd>,
    ) -> ExitStatus {
        macro_rules! start {
            ($b: expr) => {{
                match $b {
                    false => pgid,
                    true => match self.fork(pgid) {
                        Err(e) => {
                            eprintln!("{}: {}", e.name(), e.desc());
                            return ExitStatus::failure();
                        }
                        Ok(Some(child)) => {
                            child.start(self.wrapper());
                            return self.start_job(child.pid());
                        }
                        Ok(None) => {
                            self.handler = JobSignalHandler::start().unwrap();
                            Some(pgid.unwrap_or(self.wrapper().getpid()))
                        }
                    },
                }
            }};
        }

        match cmd {
            UnitKind::SimpleCommand {
                command,
                redirect,
                background,
            } => self
                .execute_simple_command(command, redirect, background, pgid, pipe_read, pipe_write),
            UnitKind::Connecter {
                left,
                right,
                kind,
                background,
            } => {
                let pgid = start!(background);
                let ret = self.execute_connecter(left, right, kind, pgid);
                match background {
                    false => ret,
                    true => self.wrapper().exit(ret.code()),
                }
            }
            UnitKind::If {
                condition,
                true_case,
                false_case,
                redirect,
                background,
            } => {
                let pgid = start!(background);
                let ret = self
                    .execute_if_command(condition, true_case, false_case, redirect, false, pgid);
                match background {
                    false => ret,
                    true => self.wrapper().exit(ret.code()),
                }
            }
            UnitKind::Unless {
                condition,
                false_case,
                true_case,
                redirect,
                background,
            } => {
                let pgid = start!(background);
                let ret =
                    self.execute_if_command(condition, false_case, true_case, redirect, true, pgid);
                match background {
                    false => ret,
                    true => self.wrapper().exit(ret.code()),
                }
            }
            UnitKind::While {
                condition,
                command,
                redirect,
                background,
            } => {
                let pgid = start!(background);
                let ret = self.execute_while_command(condition, command, redirect, false, pgid);
                match background {
                    false => ret,
                    true => self.wrapper().exit(ret.code()),
                }
            }
            UnitKind::Until {
                condition,
                command,
                redirect,
                background,
            } => {
                let pgid = start!(background);
                let ret = self.execute_while_command(condition, command, redirect, true, pgid);
                match background {
                    false => ret,
                    true => self.wrapper().exit(ret.code()),
                }
            }
            UnitKind::For {
                identifier,
                list,
                command,
                redirect,
                background,
            } => {
                let pgid = start!(background);
                let ret = self.execute_for_command(identifier, list, command, redirect, pgid);
                match background {
                    false => ret,
                    true => self.wrapper().exit(ret.code()),
                }
            }
        }
    }

    fn execute_simple_command(
        &mut self,
        command: Vec<WordList>,
        redirect: RedirectList,
        background: bool,
        pgid: Option<Pid>,
        _pipe_read: Option<RawFd>,
        _pipe_write: Option<RawFd>,
    ) -> ExitStatus {
        let (temp_env, cmds) = split_env_and_commands(&self.ctx, command);
        if cmds.is_empty() && temp_env.is_present() {
            temp_env.iter().for_each(|(k, v)| self.ctx.set_var(k, v));
            ExitStatus::success()
        } else if cmds.is_present() {
            match self.fork(pgid) {
                Err(e) => {
                    eprintln!("{}: {}", e.name(), e.desc());
                    ExitStatus::failure()
                }
                Ok(Some(child)) => {
                    if background {
                        child.start(self.wrapper());
                        self.start_job(child.pid())
                    } else {
                        let old_pgrp =
                            if pgid.is_none() && self.wrapper().isatty(0).unwrap_or(false) {
                                let old = self.wrapper().tcgetpgrp(0).ok();
                                self.wrapper().tcsetpgrp(0, child.pid()).ok();
                                old
                            } else {
                                None
                            };

                        child.start(self.wrapper());
                        let ret = self
                            .handler
                            .wait_for(child.pid(), true)
                            .unwrap_or(ExitStatus::failure());

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
        left: Box<UnitKind>,
        right: Box<UnitKind>,
        kind: ConnecterKind,
        pgid: Option<Pid>,
    ) -> ExitStatus {
        match kind {
            ConnecterKind::And => match self.execute_command_internal(*left, pgid, None, None) {
                status if status.is_error() => status,
                _ => self.execute_command_internal(*right, pgid, None, None),
            },
            ConnecterKind::Or => match self.execute_command_internal(*left, pgid, None, None) {
                status if status.is_success() => status,
                _ => self.execute_command_internal(*right, pgid, None, None),
            },
            ConnecterKind::Pipe => unimplemented![],
            ConnecterKind::PipeBoth => unimplemented![],
        }
    }

    fn execute_if_command(
        &mut self,
        condition: Box<UnitKind>,
        true_case: Vec<UnitKind>,
        false_case: Option<Vec<UnitKind>>,
        _redirect: RedirectList,
        inverse: bool,
        pgid: Option<Pid>,
    ) -> ExitStatus {
        match self.execute_command_internal(*condition, pgid, None, None) {
            status if (!inverse && status.is_success()) || (inverse && status.is_error()) => {
                let s = true_case
                    .into_iter()
                    .map(|command| self.execute_command_internal(command, pgid, None, None));
                s.last().unwrap()
            }
            status if false_case.is_none() => status,
            _ => {
                let s = false_case
                    .unwrap()
                    .into_iter()
                    .map(|command| self.execute_command_internal(command, pgid, None, None));
                s.last().unwrap()
            }
        }
    }

    fn execute_while_command(
        &mut self,
        condition: Box<UnitKind>,
        command: Vec<UnitKind>,
        _redirect: RedirectList,
        inverse: bool,
        pgid: Option<Pid>,
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
            match self.execute_command_internal(*condition.clone(), pgid, None, None) {
                status if (!inverse && status.is_success()) || (inverse && status.is_error()) => {
                    for c in command.to_vec() {
                        interrupt!();
                        self.execute_command_internal(c, pgid, None, None);
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
        command: Vec<UnitKind>,
        _redirect: RedirectList,
        pgid: Option<Pid>,
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
            None => vec![], // Normally, it returns $0.
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

                self.execute_command_internal(c, pgid, None, None);
            }
        }

        ExitStatus::new(0)
    }

    fn fork(
        &mut self,
        pgid: Option<Pid>,
    ) -> std::result::Result<Option<ChildProcess>, SysCallError> {
        let (tmp_read, tmp_write) = self.wrapper().pipe()?;

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

                let ret = ChildProcess::new(child, tmp_write);
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

    fn start_job(&mut self, pid: Pid) -> ExitStatus {
        self.job_id += 1;
        let mut job = Job::new(self.job_id, pid);
        job.append(Process::running(pid));
        self.jobs.push(job);
        println!("[{}] {}", self.job_id, pid);
        ExitStatus::success()
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

include!("exec_test.rs");
