mod mruby;
mod option;
mod redirect;

pub use redirect::SHELL_FDBASE;

use crate::{
    builtin::{builtin_command_exec, is_builtin_command},
    context::Context,
    signal::{
        change_sa_restart_flag, close_signal_handler, reset_signal_handler, restore_tty_signals,
        JobSignalHandler,
    },
    status::ExitStatus,
    syscall::{self, PrCtlFlag, SysCallError, SysCallResult},
};
use is_executable::IsExecutable;
use mruby::mruby_exec;
use nix::{
    errno::Errno,
    sys::signal::Signal,
    unistd::{ForkResult, Pid},
};
use option::{ExecOption, ExecOptionBuilder};
use reddish_parser::{
    parse_command_line, ConnecterKind, Location, Redirect, RedirectList, Unit, UnitKind, Word,
    WordKind, WordList,
};
use redirect::ApplyRedirect;
use rust_mruby::MRuby;
use std::{
    collections::HashMap,
    env,
    ffi::CString,
    fs::File,
    io::{Error as IoError, Read},
    os::unix::io::{FromRawFd, RawFd},
    path::PathBuf,
};

pub trait WordParser {
    fn to_string(self, context: &Context) -> Result<String, IoError>;
}

impl WordParser for Word {
    fn to_string(self, ctx: &Context) -> Result<String, std::io::Error> {
        let (s, k, _) = self.take();
        match k {
            WordKind::Normal | WordKind::Quote | WordKind::Literal => Ok(s),
            WordKind::Variable | WordKind::Parameter => Ok(ctx.get_var(s).unwrap_or_default()),
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

    fn start(&self) {
        syscall::close(self.pipe).ok();
    }
}

#[derive(Debug, PartialEq)]
enum SimpleCommandKind {
    Noop,
    SetEnv {
        env: Env,
    },
    Break {
        args: Args,
    },
    Continue {
        args: Args,
    },
    MRuby {
        env: Env,
        args: Args,
    },
    Builtin {
        env: Env,
        command: String,
        args: Args,
    },
    External {
        env: Env,
        command: String,
        args: Args,
    },
}

type Env = HashMap<String, String>;
type Args = Vec<String>;

pub struct Executor {
    handler: JobSignalHandler,
    pub job_id: u16,
    jobs: Vec<Job>,
    loop_level: usize,
    breaking: usize,
    continuing: usize,
    mrb: MRuby,
}

impl Executor {
    pub fn new() -> std::result::Result<Self, std::io::Error> {
        Ok(Self {
            handler: JobSignalHandler::start()?,
            job_id: 0,
            jobs: vec![],
            loop_level: 0,
            breaking: 0,
            continuing: 0,
            mrb: MRuby::new(),
        })
    }

    pub fn execute_command(
        &mut self,
        ctx: &mut Context,
        cmd: Unit,
        option: Option<ExecOption>,
    ) -> ExitStatus {
        self.handler.reset_interrupt_flag();
        self.handler.reset_forground();

        self.loop_level = 0;
        self.breaking = 0;
        self.continuing = 0;

        self.execute_command_internal(ctx, cmd, option)
    }

    fn execute_command_internal(
        &mut self,
        ctx: &mut Context,
        cmd: Unit,
        option: Option<ExecOption>,
    ) -> ExitStatus {
        let option = option.unwrap_or_else(|| ExecOptionBuilder::new().build());

        let ret = match cmd.kind() {
            UnitKind::SimpleCommand { command, redirect } => {
                let ret =
                    self.execute_simple_command(ctx, command, redirect, cmd.background(), option);
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
                                child.start();
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
                                let pgid = pgid.unwrap_or_else(syscall::getpid);
                                ExecOptionBuilder::from(option).pgid(pgid).build()
                            }
                        }
                    }
                };

                if let Some(fd) = option.leak_fd() {
                    match close(fd) {
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
                        self.execute_connecter(ctx, *left, *right, kind, option)
                    }
                    UnitKind::Pipe { left, right, both } => {
                        self.execute_pipe(ctx, *left, *right, both, option)
                    }
                    UnitKind::If {
                        condition,
                        true_case,
                        false_case,
                        redirect,
                    } => self.execute_if_command(
                        ctx, *condition, true_case, false_case, redirect, false, option,
                    ),
                    UnitKind::Unless {
                        condition,
                        false_case,
                        true_case,
                        redirect,
                    } => self.execute_if_command(
                        ctx, *condition, false_case, true_case, redirect, true, option,
                    ),
                    UnitKind::While {
                        condition,
                        command,
                        redirect,
                    } => self
                        .execute_while_command(ctx, *condition, command, redirect, false, option),
                    UnitKind::Until {
                        condition,
                        command,
                        redirect,
                    } => {
                        self.execute_while_command(ctx, *condition, command, redirect, true, option)
                    }
                    UnitKind::For {
                        identifier,
                        list,
                        command,
                        redirect,
                    } => self.execute_for_command(ctx, identifier, list, command, redirect, option),
                };

                match background {
                    false => ret,
                    true => syscall::exit(ret.code()),
                }
            }
        };

        ctx.status = ret;
        ret
    }

    fn execute_simple_command(
        &mut self,
        ctx: &mut Context,
        command: Vec<WordList>,
        mut redirect: RedirectList,
        background: bool,
        option: ExecOption,
    ) -> ExitStatus {
        let kind = match expand_command_line(ctx, command) {
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

        let background = background || option.piping();
        let need_fork = matches!(
            kind,
            SimpleCommandKind::MRuby { .. } | SimpleCommandKind::External { .. }
        );
        if background || need_fork {
            let pgid = option.pgid();
            match self.fork(pgid) {
                Err(e) => {
                    eprintln!("{}: {}", e.name(), e.desc());
                    return ExitStatus::failure();
                }
                Ok(Some(child)) => match background {
                    true => {
                        child.start();
                        return self.start_job(child.pid(), child.pgid());
                    }
                    false => {
                        let old_pgrp = match syscall::isatty(0).unwrap_or(false) {
                            false => None,
                            true => match pgid {
                                Some(_) => None,
                                None => syscall::tcgetpgrp(0)
                                    .map(|old| {
                                        syscall::tcsetpgrp(0, child.pgid()).ok();
                                        old
                                    })
                                    .ok(),
                            },
                        };

                        self.handler.set_forground(child.pgid());
                        self.start_job(child.pid(), child.pgid());
                        child.start();
                        let status = self
                            .handler
                            .wait_for(child.pid(), true)
                            .unwrap_or_else(ExitStatus::failure);
                        self.handler.reset_forground();
                        self.jobs.pop(); // remove current job

                        if let Some(pgid) = old_pgrp {
                            syscall::tcsetpgrp(0, pgid).ok();
                        }

                        return status;
                    }
                },
                Ok(None) => (),
            }
        }

        if let Some(pipe) = option.input() {
            redirect.insert(0, Redirect::copy(pipe, 0, true, Location::new(0, 0)));
        }
        if let Some((pipe, both)) = option.output() {
            redirect.insert(0, Redirect::copy(pipe, 1, true, Location::new(0, 0)));
            if both {
                redirect.push(Redirect::copy(1, 2, false, Location::new(0, 0)));
            }
        }

        if let Some(fd) = option.leak_fd() {
            syscall::close(fd).ok();
        }

        let restore = match redirect.apply(ctx, !(background || need_fork)) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("{:?}", e);
                return match need_fork {
                    false => ExitStatus::failure(),
                    true => syscall::exit(1),
                };
            }
        };

        let status = match kind {
            SimpleCommandKind::Noop => ExitStatus::success(),
            SimpleCommandKind::SetEnv { env } => {
                env.iter().for_each(|(k, v)| {
                    ctx.set_var(k, v);
                });
                ExitStatus::success()
            }
            SimpleCommandKind::Break { .. } | SimpleCommandKind::Continue { .. } => {
                self.do_break_or_continue(kind)
            }
            SimpleCommandKind::MRuby { env, args } => {
                let name = CString::new("mruby").unwrap();
                if let Some(e) =
                    syscall::prctl(PrCtlFlag::PR_SET_NAME, name.as_ptr() as nix::libc::c_ulong)
                        .err()
                {
                    eprintln!("mruby: prctl: {}", e.desc());
                }

                match reset_signal_handler() {
                    Ok(_) => {
                        env.iter().for_each(|(k, v)| {
                            ctx.set_var(k, v);
                        });

                        syscall::exit(mruby_exec(&self.mrb, &args).code())
                    }
                    Err(e) => {
                        eprintln!("mruby: {}", e);
                        ExitStatus::failure()
                    }
                }
            }
            SimpleCommandKind::Builtin { env, command, args } => {
                let old_env_vars = env
                    .iter()
                    .map(|(k, v)| {
                        let old_var = ctx.set_var(k, v);
                        (k, old_var)
                    })
                    .collect::<Vec<_>>();

                let status = builtin_command_exec(ctx, command, &args);

                old_env_vars.iter().for_each(|(k, v)| {
                    match v {
                        None => ctx.unset_var(k),
                        Some(v) => ctx.set_var(k, &v),
                    };
                });

                status
            }
            SimpleCommandKind::External { env, command, args } => {
                execute_external_command(env, command, args)
            }
        };

        restore.apply(ctx, false).ok();

        match background {
            true => syscall::exit(status.code()),
            false => status,
        }
    }

    fn do_break_or_continue(&mut self, kind: SimpleCommandKind) -> ExitStatus {
        let (command, args) = match kind {
            SimpleCommandKind::Break { args } => ("break", args),
            SimpleCommandKind::Continue { args } => ("continue", args),
            _ => unreachable![],
        };

        if self.loop_level == 0 {
            eprintln!(
                "reddish: {}: only meaningful in a `for', `while', or `until' loop",
                command
            );
            return ExitStatus::failure();
        }

        let count = match args.len() {
            0 => Some(self.loop_level),
            1 => args[0].parse::<usize>().map_or_else(
                |e| {
                    eprintln!("reddish: {}: {}", command, e);
                    None
                },
                Some,
            ),
            _ => {
                eprintln!("reddish: {}: too many arguments", command);
                None
            }
        };

        match count {
            None => ExitStatus::failure(),
            Some(c) => {
                let c = match c > self.loop_level {
                    true => self.loop_level,
                    false => c,
                };
                match command {
                    "break" => self.breaking = c,
                    "continue" => self.continuing = c,
                    _ => unreachable![],
                }
                ExitStatus::success()
            }
        }
    }

    fn execute_connecter(
        &mut self,
        ctx: &mut Context,
        left: Unit,
        right: Unit,
        kind: ConnecterKind,
        option: ExecOption,
    ) -> ExitStatus {
        let option = ExecOptionBuilder::from(option).piping(false).build();
        let condition = self.execute_command_internal(ctx, left, Some(option));

        let option = ExecOptionBuilder::from(option)
            .input(None)
            .output(None)
            .build();
        match kind {
            ConnecterKind::And if condition.is_success() => {
                self.execute_command_internal(ctx, right, Some(option))
            }
            ConnecterKind::Or if condition.is_error() => {
                self.execute_command_internal(ctx, right, Some(option))
            }
            _ => condition,
        }
    }

    fn execute_pipe(
        &mut self,
        ctx: &mut Context,
        left: Unit,
        right: Unit,
        both: bool,
        option: ExecOption,
    ) -> ExitStatus {
        let (pipe_read, pipe_write) = pipe().unwrap();

        self.execute_command_internal(
            ctx,
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
        if !piping && syscall::isatty(0).unwrap_or(false) {
            let job = self.jobs.last().unwrap();
            syscall::tcsetpgrp(0, job.pgid).ok();
        }

        self.execute_command_internal(
            ctx,
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

        close(pipe_read).unwrap();
        close(pipe_write).unwrap();

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

            if !piping && syscall::isatty(0).unwrap_or(false) {
                let pgid = syscall::getpgid(None).unwrap();
                syscall::tcsetpgrp(0, pgid).ok();
            }

            statuses.last().unwrap_or(&ExitStatus::success()).to_owned()
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn execute_if_command(
        &mut self,
        ctx: &mut Context,
        condition: Unit,
        true_case: Vec<Unit>,
        false_case: Option<Vec<Unit>>,
        redirect: RedirectList,
        inverse: bool,
        option: ExecOption,
    ) -> ExitStatus {
        let (restore, option) = self.update_option_and_apply_redirect(ctx, option, redirect);

        let ret = match self.execute_command_internal(ctx, condition, Some(option)) {
            status if (!inverse && status.is_success()) || (inverse && status.is_error()) => {
                let s = true_case
                    .into_iter()
                    .map(|command| self.execute_command_internal(ctx, command, Some(option)));
                s.last().unwrap()
            }
            status if false_case.is_none() => status,
            _ => {
                let s = false_case
                    .unwrap()
                    .into_iter()
                    .map(|command| self.execute_command_internal(ctx, command, Some(option)));
                s.last().unwrap()
            }
        };

        restore.apply(ctx, false).unwrap();
        ret
    }

    fn execute_while_command(
        &mut self,
        ctx: &mut Context,
        condition: Unit,
        command: Vec<Unit>,
        redirect: RedirectList,
        inverse: bool,
        option: ExecOption,
    ) -> ExitStatus {
        let (restore, option) = self.update_option_and_apply_redirect(ctx, option, redirect);
        self.loop_level += 1;

        'exec: loop {
            macro_rules! interrupt {
                () => {
                    if self.handler.is_interrupt() {
                        self.breaking = self.loop_level - 1;
                        break 'exec;
                    }
                };
            }
            macro_rules! break_or_continue {
                () => {
                    if self.breaking > 0 {
                        self.breaking -= 1;
                        break 'exec;
                    }

                    if self.continuing > 0 {
                        self.continuing -= 1;
                        if self.continuing > 0 {
                            break 'exec;
                        } else {
                            continue 'exec;
                        }
                    }
                };
            }

            interrupt!();
            let status = self.execute_command_internal(ctx, condition.clone(), Some(option));
            break_or_continue!();

            if (!inverse && status.is_success()) || (inverse && status.is_error()) {
                for c in command.iter().cloned() {
                    interrupt!();
                    self.execute_command_internal(ctx, c, Some(option));

                    break_or_continue!();
                }
            } else {
                break 'exec;
            }
        }

        self.loop_level -= 1;
        restore.apply(ctx, false).unwrap();
        ExitStatus::new(0)
    }

    fn execute_for_command(
        &mut self,
        ctx: &mut Context,
        identifier: Word,
        list: Option<Vec<WordList>>,
        command: Vec<Unit>,
        redirect: RedirectList,
        option: ExecOption,
    ) -> ExitStatus {
        let identifier = match identifier.take() {
            (string, kind, _) if kind == WordKind::Normal => string,
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
                .map(|w| w.to_string(ctx).unwrap())
                .collect::<Vec<_>>(),
        };

        let (restore, option) = self.update_option_and_apply_redirect(ctx, option, redirect);
        self.loop_level += 1;
        'exec: for word in list.iter() {
            ctx.set_var(&*identifier, &*word);
            for c in command.iter().cloned() {
                if self.handler.is_interrupt() {
                    break 'exec;
                }

                self.execute_command_internal(ctx, c, Some(option));

                if self.breaking > 0 {
                    self.breaking -= 1;
                    break 'exec;
                }

                if self.continuing > 0 {
                    self.continuing -= 1;
                    if self.continuing > 0 {
                        break 'exec;
                    }
                }
            }
        }
        self.loop_level -= 1;
        restore.apply(ctx, false).ok();

        ExitStatus::new(0)
    }

    fn fork(
        &mut self,
        pgid: Option<Pid>,
    ) -> std::result::Result<Option<ChildProcess>, SysCallError> {
        let (tmp_read, tmp_write) = pipe()?;
        match syscall::fork() {
            Err(e) => {
                syscall::close(tmp_read).ok();
                syscall::close(tmp_write).ok();
                Err(e)
            }
            Ok(ForkResult::Parent { child }) => {
                let new_pgid = pgid.unwrap_or(child);
                syscall::setpgid(child, new_pgid).ok();
                syscall::close(tmp_read).ok();

                let ret = ChildProcess::new(child, new_pgid, tmp_write);
                Ok(Some(ret))
            }
            Ok(ForkResult::Child) => {
                close_signal_handler();

                syscall::close(tmp_write).ok();

                // Synchronize with the parent process.
                loop {
                    let mut buf = [0];
                    match syscall::read(tmp_read, &mut buf) {
                        // Read again because it was interrupted by Signal.
                        Err(e) if e.errno() == nix::errno::Errno::EINTR => (),
                        _ => break,
                    }
                }
                syscall::close(tmp_read).ok();

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
        ctx: &Context,
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
            redirect.apply(ctx, !option.piping()).unwrap_or_default(),
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
        let (pipe_read, pipe_write) = pipe().unwrap();

        match syscall::fork() {
            Err(e) => {
                eprintln!("{}: {}", e.name(), e.desc());
                Err(IoError::from_raw_os_error(e.errno() as i32))
            }
            Ok(ForkResult::Parent { child }) => {
                syscall::setpgid(child, child).unwrap();
                syscall::close(pipe_write).unwrap();
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
                syscall::close(pipe_read).unwrap();
                change_sa_restart_flag(true)?;

                let pgid = Pid::from_raw(-child.as_raw());
                syscall::waitpid(pgid, None).ok();

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

                let pid = syscall::getpid();
                syscall::setpgid(pid, pid).unwrap();
                let old_pgrp = match syscall::isatty(0).unwrap_or(false) {
                    false => None,
                    true => syscall::tcgetpgrp(0)
                        .map(|old| {
                            syscall::tcsetpgrp(0, pid).ok();
                            old
                        })
                        .ok(),
                };

                syscall::close(pipe_read).unwrap();
                syscall::dup2(pipe_write, 1).unwrap();
                syscall::close(pipe_write).unwrap();

                let mut e = Executor::new().unwrap();
                let option = ExecOptionBuilder::new().quiet(true).pgid(pid).build();
                let status = match parse_command_line(command, 0, ctx.debug()) {
                    Err(_) => ExitStatus::failure(),
                    Ok(cmds) => {
                        for cmd in cmds.to_vec() {
                            e.execute_command(&mut ctx.clone(), cmd, Some(option));
                        }
                        if let Some(pgrp) = old_pgrp {
                            syscall::tcsetpgrp(0, pgrp).unwrap();
                        }
                        ExitStatus::success()
                    }
                };
                syscall::exit(status.code());

                unreachable![]
            }
        }
    }

    pub fn close(self) {
        self.handler.close()
    }
}

fn expand_command_line(ctx: &Context, list: Vec<WordList>) -> Result<SimpleCommandKind, IoError> {
    if list.is_empty() {
        return Ok(SimpleCommandKind::Noop);
    }

    let mut env = Env::new();
    let mut iter = list.into_iter().peekable();
    loop {
        match iter.peek() {
            Some(wl) if wl.is_var_name() => {
                let wl = iter.next().unwrap();
                let s = wl.to_string(ctx)?;
                let (k, v) = s.split_once('=').unwrap();
                env.insert(k.to_string(), v.to_string());
            }
            _ => break,
        }
    }

    if iter.peek().is_none() {
        return Ok(SimpleCommandKind::SetEnv { env });
    }

    let mut cmds = vec![];
    for wl in iter {
        let s = wl.to_string(ctx)?;
        cmds.push(s);
    }

    let command = cmds.remove(0);
    let args = cmds;

    match &*command {
        "break" => Ok(SimpleCommandKind::Break { args }),
        "continue" | "next" => Ok(SimpleCommandKind::Continue { args }),
        "mruby" => Ok(SimpleCommandKind::MRuby { env, args }),
        _ => match is_builtin_command(&command) {
            true => Ok(SimpleCommandKind::Builtin { env, command, args }),
            false => Ok(SimpleCommandKind::External { env, command, args }),
        },
    }
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

fn execute_external_command(
    env: HashMap<String, String>,
    command: String,
    args: Vec<String>,
) -> ExitStatus {
    let cmdpath = assume_command(&*command).to_str().unwrap().to_cstring();
    let mut cmds = vec![command.to_cstring()];
    cmds.append(&mut args.to_cstring());

    let env = {
        let mut t = syscall::env_vars();
        t.extend(env);
        t
    }
    .into_iter()
    .map(|(k, v)| format!("{}={}", k, v).to_cstring())
    .collect::<Vec<_>>();

    restore_tty_signals().unwrap();

    match syscall::execve(cmdpath, &cmds, &env) {
        Ok(_) => unreachable![],
        Err(e) if e.errno() == Errno::ENOENT => {
            eprintln!("{}: command not found", command);
            syscall::exit(127)
        }
        Err(e) => {
            eprintln!("execve faile: {}", e.errno());
            syscall::exit(1)
        }
    }
}

fn pipe() -> SysCallResult<(RawFd, RawFd)> {
    let (tmp_read, tmp_write) = syscall::pipe()?;
    let read = syscall::dup_fd(tmp_read, SHELL_FDBASE)?;
    let write = syscall::dup_fd(tmp_write, SHELL_FDBASE)?;
    syscall::close(tmp_read)?;
    syscall::close(tmp_write)?;
    Ok((read, write))
}

fn close(fd: RawFd) -> SysCallResult<()> {
    match syscall::close(fd) {
        Ok(_) => Ok(()),
        Err(e) if e.errno() == Errno::EBADF => Ok(()),
        Err(e) => Err(e),
    }
}

include!("exec_test.rs");
