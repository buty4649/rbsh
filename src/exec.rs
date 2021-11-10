mod option;
mod redirect;

pub mod syscall;
pub use redirect::SHELL_FDBASE;
pub use syscall::SysCallError;

use super::{
    context::Context,
    error::ShellErrorKind,
    location::Location,
    parse_command_line,
    parser::{
        redirect::{Redirect, RedirectList},
        word::{Word, WordKind, WordList},
        ConnecterKind, Unit, UnitKind,
    },
    signal::{close_signal_handler, restore_tty_signals, JobSignalHandler},
    status::ExitStatus,
};
use is_executable::IsExecutable;
use nix::{
    errno::{errno, Errno},
    libc,
    sys::signal::Signal,
    unistd::{ForkResult, Pid},
};
use option::{ExecOption, ExecOptionBuilder};
use redirect::ApplyRedirect;
use std::{
    collections::HashMap,
    env,
    ffi::CString,
    fs::File,
    io::{Error as IoError, Read},
    mem,
    os::unix::io::{FromRawFd, RawFd},
    path::PathBuf,
    ptr,
};
use syscall::{SysCallResult, SysCallWrapper, Wrapper};

pub trait WordParser {
    fn to_string(self, context: &Context) -> Result<String, IoError>;
}

impl WordParser for Word {
    fn to_string(self, ctx: &Context) -> Result<String, std::io::Error> {
        let (s, k, _) = self.take();
        match k {
            WordKind::Normal | WordKind::Quote | WordKind::Literal => Ok(s),
            WordKind::Variable | WordKind::Parameter => {
                Ok(ctx.get_var(s).unwrap_or_else(String::new))
            }
            WordKind::Command => Executor::capture_command_output(ctx, s),
        }
    }
}

impl WordParser for WordList {
    fn to_string(self, ctx: &Context) -> Result<String, IoError> {
        let mut result = String::new();
        for word in self.to_vec() {
            let s = word.to_string(ctx)?;
            result.push_str(&*s);
        }
        Ok(result)
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
    pub fn new(ctx: Context) -> std::result::Result<Self, std::io::Error> {
        Ok(Self {
            ctx,
            handler: JobSignalHandler::start()?,
            job_id: 0,
            jobs: vec![],
        })
    }

    pub fn context(&self) -> &Context {
        &self.ctx
    }

    fn wrapper(&self) -> &Wrapper {
        self.ctx.wrapper()
    }

    pub fn execute_command(&mut self, cmd: Unit, option: Option<ExecOption>) -> ExitStatus {
        let option = option.unwrap_or_else(|| ExecOptionBuilder::new().build());

        let ret = match cmd.kind() {
            UnitKind::SimpleCommand { command, redirect } => {
                let ret = self.execute_simple_command(command, redirect, cmd.background(), option);
                if cmd.background() {
                    let job = self.jobs.last().unwrap();
                    if option.verbose() {
                        println!("[{}] {}", job.id, job.pgid);
                    }
                }
                ret
            }
            kind => {
                let background = cmd.background()
                    || match kind {
                        UnitKind::Pipe { .. } => false,
                        _ => option.piping(),
                    };
                let option = match background {
                    false => option,
                    true => {
                        let pgid = option.pgid();
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
                                    if option.verbose() {
                                        println!("[{}] {}", job.id, job.pgid);
                                    }
                                }
                                return ret;
                            }
                            Ok(None) => {
                                self.handler = JobSignalHandler::start().unwrap();
                                let pgid = pgid.unwrap_or_else(|| self.wrapper().getpid());
                                ExecOptionBuilder::from(option).pgid(pgid).build()
                            }
                        }
                    }
                };

                if let Some(fd) = option.leak_fd() {
                    match close(&self.ctx, fd) {
                        Ok(_) => (),
                        Err(e) => {
                            eprintln!("{}, {}", e.name(), e.desc());
                            return ExitStatus::failure();
                        }
                    }
                }

                let ret = match kind {
                    UnitKind::SimpleCommand {
                        command: _,
                        redirect: _,
                    } => unreachable![],
                    UnitKind::Connecter { left, right, kind } => {
                        self.execute_connecter(*left, *right, kind, option)
                    }
                    UnitKind::Pipe { left, right, both } => {
                        self.execute_pipe(*left, *right, both, option)
                    }
                    UnitKind::If {
                        condition,
                        true_case,
                        false_case,
                        redirect,
                    } => self.execute_if_command(
                        *condition, true_case, false_case, redirect, false, option,
                    ),
                    UnitKind::Unless {
                        condition,
                        false_case,
                        true_case,
                        redirect,
                    } => self.execute_if_command(
                        *condition, false_case, true_case, redirect, true, option,
                    ),
                    UnitKind::While {
                        condition,
                        command,
                        redirect,
                    } => self.execute_while_command(*condition, command, redirect, false, option),
                    UnitKind::Until {
                        condition,
                        command,
                        redirect,
                    } => self.execute_while_command(*condition, command, redirect, true, option),
                    UnitKind::For {
                        identifier,
                        list,
                        command,
                        redirect,
                    } => self.execute_for_command(identifier, list, command, redirect, option),
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
        redirect: RedirectList,
        background: bool,
        option: ExecOption,
    ) -> ExitStatus {
        let (temp_env, cmds) = match split_env_and_commands(&self.ctx, command) {
            Ok(ret) => ret,
            Err(e) => {
                return match e.kind() {
                    std::io::ErrorKind::Interrupted => ExitStatus::signaled(Signal::SIGINT),
                    e => {
                        eprintln!("{:?}", e);
                        ExitStatus::failure()
                    }
                }
            }
        };
        if cmds.is_empty() && temp_env.is_present() {
            temp_env.iter().for_each(|(k, v)| self.ctx.set_var(k, v));
            ExitStatus::success()
        } else if cmds.is_present() {
            let pgid = option.pgid();
            match self.fork(pgid) {
                Err(e) => {
                    eprintln!("{}: {}", e.name(), e.desc());
                    ExitStatus::failure()
                }
                Ok(Some(child)) => {
                    let background = background || option.piping();
                    if background {
                        child.start(self.wrapper());
                        self.start_job(child.pid(), pgid.unwrap_or(child.pgid))
                    } else {
                        let need_terminal = !background && pgid.is_none();
                        if need_terminal && self.wrapper().isatty(0).unwrap_or(false) {
                            self.wrapper().tcsetpgrp(0, child.pgid()).ok();
                        }

                        self.handler.set_forground(child.pgid());
                        child.start(self.wrapper());
                        self.start_job(child.pid(), child.pgid());
                        let ret = self
                            .handler
                            .wait_for(child.pid(), true)
                            .unwrap_or_else(ExitStatus::failure);
                        self.handler.reset_forground();

                        self.jobs.pop(); // remove current job

                        if need_terminal && self.wrapper().isatty(0).unwrap_or(false) {
                            let pgid = self.wrapper().getpgid(None).unwrap();
                            self.wrapper().tcsetpgrp(0, pgid).ok();
                        }

                        ret
                    }
                }
                Ok(None) => {
                    if let Some(pipe) = option.input() {
                        self.wrapper().dup2(pipe, 0).unwrap();
                        self.wrapper().close(pipe).unwrap();
                    }
                    if let Some((pipe, both)) = option.output() {
                        self.wrapper().dup2(pipe, 1).unwrap();
                        if both {
                            self.wrapper().dup2(1, 2).unwrap();
                        }
                        self.wrapper().close(pipe).unwrap();
                    }

                    if let Some(fd) = option.leak_fd() {
                        self.wrapper().close(fd).ok();
                    }

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
        left: Unit,
        right: Unit,
        kind: ConnecterKind,
        option: ExecOption,
    ) -> ExitStatus {
        let option = ExecOptionBuilder::from(option).piping(false).build();
        let condition = self.execute_command(left, Some(option));

        let option = ExecOptionBuilder::from(option)
            .input(None)
            .output(None)
            .build();
        match kind {
            ConnecterKind::And if condition.is_success() => {
                self.execute_command(right, Some(option))
            }
            ConnecterKind::Or if condition.is_error() => self.execute_command(right, Some(option)),
            _ => condition,
        }
    }

    fn execute_pipe(
        &mut self,
        left: Unit,
        right: Unit,
        both: bool,
        option: ExecOption,
    ) -> ExitStatus {
        let (pipe_read, pipe_write) = pipe(&self.ctx).unwrap();

        self.execute_command(
            left,
            Some(
                ExecOptionBuilder::from(option)
                    .piping(true)
                    .output(Some(pipe_write))
                    .leak_fd(Some(pipe_read))
                    .if_then(both, |b| b.both_output())
                    .build(),
            ),
        );

        let piping = option.piping();
        if !piping && self.wrapper().isatty(0).unwrap_or(false) {
            let job = self.jobs.last().unwrap();
            self.wrapper().tcsetpgrp(0, job.pgid).ok();
        }

        self.execute_command(
            right,
            Some(
                ExecOptionBuilder::from(option)
                    .piping(true)
                    .default_pgid(self.jobs.last().unwrap().pgid)
                    .input(Some(pipe_read))
                    .leak_fd(Some(pipe_write))
                    .build(),
            ),
        );

        close(&self.ctx, pipe_read).unwrap();
        close(&self.ctx, pipe_write).unwrap();

        if piping {
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
                        .unwrap_or_else(ExitStatus::failure)
                })
                .collect::<Vec<_>>();
            self.handler.reset_forground();

            if !piping && self.wrapper().isatty(0).unwrap_or(false) {
                let pgid = self.wrapper().getpgid(None).unwrap();
                self.wrapper().tcsetpgrp(0, pgid).ok();
            }

            statuses.last().unwrap_or(&ExitStatus::success()).to_owned()
        }
    }

    fn execute_if_command(
        &mut self,
        condition: Unit,
        true_case: Vec<Unit>,
        false_case: Option<Vec<Unit>>,
        redirect: RedirectList,
        inverse: bool,
        option: ExecOption,
    ) -> ExitStatus {
        let (restore, option) = self.update_option_and_apply_redirect(option, redirect);

        let ret = match self.execute_command(condition, Some(option)) {
            status if (!inverse && status.is_success()) || (inverse && status.is_error()) => {
                let s = true_case
                    .into_iter()
                    .map(|command| self.execute_command(command, Some(option)));
                s.last().unwrap()
            }
            status if false_case.is_none() => status,
            _ => {
                let s = false_case
                    .unwrap()
                    .into_iter()
                    .map(|command| self.execute_command(command, Some(option)));
                s.last().unwrap()
            }
        };

        restore.apply(&self.ctx, false).unwrap();
        ret
    }

    fn execute_while_command(
        &mut self,
        condition: Unit,
        command: Vec<Unit>,
        redirect: RedirectList,
        inverse: bool,
        option: ExecOption,
    ) -> ExitStatus {
        let (restore, option) = self.update_option_and_apply_redirect(option, redirect);

        'exec: loop {
            macro_rules! interrupt {
                () => {
                    if self.handler.is_interrupt() {
                        break 'exec;
                    }
                };
            }

            interrupt!();
            match self.execute_command(condition.clone(), Some(option)) {
                status if (!inverse && status.is_success()) || (inverse && status.is_error()) => {
                    for c in command.to_vec() {
                        interrupt!();
                        self.execute_command(c, Some(option));
                    }
                }
                _ => break 'exec,
            }
        }

        restore.apply(&self.ctx, false).unwrap();
        ExitStatus::new(0)
    }

    fn execute_for_command(
        &mut self,
        identifier: Word,
        list: Option<Vec<WordList>>,
        command: Vec<Unit>,
        redirect: RedirectList,
        option: ExecOption,
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
                .map(|w| w.to_string(&self.ctx).unwrap())
                .collect::<Vec<_>>(),
        };

        let (restore, option) = self.update_option_and_apply_redirect(option, redirect);
        'exec: for word in list.iter() {
            self.ctx.set_var(&*identifier, &*word);
            for c in command.to_vec() {
                if self.handler.is_interrupt() {
                    break 'exec;
                }

                self.execute_command(c, Some(option));
            }
        }
        restore.apply(&self.ctx, false).ok();

        ExitStatus::new(0)
    }

    fn fork(
        &mut self,
        pgid: Option<Pid>,
    ) -> std::result::Result<Option<ChildProcess>, SysCallError> {
        let (tmp_read, tmp_write) = pipe(&self.ctx)?;
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
                close_signal_handler();

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
                    println!("[{}]+ Done", job.id);
                    false
                } else {
                    // remove job
                    true
                }
            })
            .cloned()
            .collect::<Vec<_>>();
        self.jobs.sort_by(|a, b| a.id.cmp(&b.id));
        self.job_id = self.jobs.last().map(|job| job.id).unwrap_or(0);
    }

    fn update_option_and_apply_redirect(
        &self,
        option: ExecOption,
        mut redirect: RedirectList,
    ) -> (RedirectList, ExecOption) {
        if let Some(pipe) = option.input() {
            let mut r = vec![Redirect::copy(pipe, 0, true, Location::new(0, 0))];
            r.append(&mut redirect);
            redirect = r;
        }
        if let Some((pipe, both)) = option.output() {
            let mut r = vec![Redirect::copy(pipe, 1, true, Location::new(0, 0))];
            if both {
                r.push(Redirect::copy(1, 2, false, Location::new(0, 0)));
            }
            r.append(&mut redirect);
            redirect = r;
        }

        (
            redirect
                .apply(&self.ctx, !option.piping())
                .unwrap_or_default(),
            ExecOptionBuilder::from(option)
                .input(None)
                .output(None)
                .piping(false)
                .build(),
        )
    }

    fn capture_command_output<I: AsRef<str>>(
        ctx: &Context,
        command: I,
    ) -> Result<String, std::io::Error> {
        let (pipe_read, pipe_write) = ctx.wrapper().pipe().unwrap();

        match ctx.wrapper().fork() {
            Err(e) => {
                eprintln!("{}: {}", e.name(), e.desc());
                Err(IoError::from_raw_os_error(e.errno() as i32))
            }
            Ok(ForkResult::Parent { child }) => {
                ctx.wrapper().setpgid(child, child).unwrap();
                ctx.wrapper().close(pipe_write).unwrap();
                change_sa_restart_flag(false)?;

                let mut pipe = unsafe { File::from_raw_fd(pipe_read) };
                let mut output = Vec::new();
                let read_result = loop {
                    let mut buf = [0u8; 4096];
                    match pipe.read(&mut buf) {
                        Ok(s) => {
                            if s == 0 {
                                break Ok(());
                            } else {
                                output.append(&mut (buf[0..s].to_vec()));
                            }
                        }
                        Err(e) => break Err(e),
                    }
                };
                ctx.wrapper().close(pipe_read).unwrap();
                change_sa_restart_flag(true)?;

                let pgid = Pid::from_raw(-child.as_raw());
                ctx.wrapper().waitpid(pgid, None).ok();

                match read_result {
                    Ok(_) => {
                        let s = std::str::from_utf8(&output).unwrap().trim_end_matches('\n');
                        Ok(s.to_string())
                    }
                    Err(e) => Err(e),
                }
            }
            Ok(ForkResult::Child) => {
                close_signal_handler();

                let pid = ctx.wrapper().getpid();
                ctx.wrapper().setpgid(pid, pid).unwrap();
                let old_pgrp = match ctx.wrapper().isatty(0).unwrap_or(false) {
                    false => None,
                    true => ctx
                        .wrapper()
                        .tcgetpgrp(0)
                        .map(|old| {
                            ctx.wrapper().tcsetpgrp(0, pid).ok();
                            old
                        })
                        .ok(),
                };

                ctx.wrapper().close(pipe_read).unwrap();
                ctx.wrapper().dup2(pipe_write, 1).unwrap();
                ctx.wrapper().close(pipe_write).unwrap();

                let mut e = Executor::new(ctx.clone()).unwrap();
                let option = ExecOptionBuilder::new().quiet(true).pgid(pid).build();
                let status = match parse_command_line(command) {
                    Err(_) => ExitStatus::failure(),
                    Ok(cmds) => {
                        for cmd in cmds.to_vec() {
                            e.execute_command(cmd, Some(option));
                        }
                        if let Some(pgrp) = old_pgrp {
                            ctx.wrapper().tcsetpgrp(0, pgrp).unwrap();
                        }
                        ExitStatus::success()
                    }
                };
                ctx.wrapper().exit(status.code());

                unreachable![]
            }
        }
    }
}

fn split_env_and_commands(
    ctx: &Context,
    list: Vec<WordList>,
) -> Result<(HashMap<String, String>, Vec<String>), IoError> {
    let mut env = HashMap::new();
    let mut cmds = vec![];
    let mut iter = list.into_iter().peekable();

    loop {
        match iter.peek() {
            Some(wl) if wl.is_var_name() => {
                let wl = iter.next().unwrap();
                let s = wl.to_string(ctx)?;
                let (k, v) = s.split_once("=").unwrap();
                env.insert(k.to_string(), v.to_string());
            }
            _ => break,
        }
    }

    for wl in iter {
        let s = wl.to_string(ctx)?;
        cmds.push(s);
    }

    Ok((env, cmds))
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
                .split(':')
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

    let mut env = env::vars().collect::<HashMap<_, _>>();
    temp_env.into_iter().for_each(|(k, v)| {
        env.insert(k, v);
    });
    let env = env
        .into_iter()
        .map(|(k, v)| format!("{}={}", k, v).to_cstring())
        .collect::<Vec<_>>();

    if let Err(e) = redirect.apply(ctx, false) {
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

fn pipe(ctx: &Context) -> SysCallResult<(RawFd, RawFd)> {
    let (tmp_read, tmp_write) = ctx.wrapper().pipe()?;
    let read = ctx.wrapper().dup_fd(tmp_read, SHELL_FDBASE)?;
    let write = ctx.wrapper().dup_fd(tmp_write, SHELL_FDBASE)?;
    ctx.wrapper().close(tmp_read)?;
    ctx.wrapper().close(tmp_write)?;
    Ok((read, write))
}

fn close(ctx: &Context, fd: RawFd) -> SysCallResult<()> {
    match ctx.wrapper().close(fd) {
        Ok(_) => Ok(()),
        Err(e) if e.errno() == Errno::EBADF => Ok(()),
        Err(e) => Err(e),
    }
}

fn change_sa_restart_flag(flag: bool) -> Result<(), IoError> {
    macro_rules! sigaction {
        ($new: expr, $old: expr) => {
            match libc::sigaction(Signal::SIGINT as libc::c_int, $new, $old) {
                -1 => Err(IoError::from_raw_os_error(errno())),
                r => Ok(r),
            }
        };
    }

    unsafe {
        let mut sa: libc::sigaction = mem::zeroed();
        sigaction!(ptr::null(), &mut sa)?;

        sa.sa_flags = match flag {
            true => sa.sa_flags | libc::SA_RESTART,
            false => sa.sa_flags & !libc::SA_RESTART,
        };
        sigaction!(&sa, ptr::null_mut())?;
    };
    Ok(())
}

include!("exec_test.rs");
