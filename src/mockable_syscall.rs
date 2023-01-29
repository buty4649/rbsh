use nix::errno::Errno;

pub type SysCallResult<T> = Result<T, SysCallError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SysCallError(String, Errno);

impl SysCallError {
    pub fn new<T: AsRef<str>>(name: T, errno: nix::errno::Errno) -> Self {
        let name = name.as_ref().to_string();
        Self(name, errno)
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn errno(&self) -> nix::errno::Errno {
        self.1
    }

    pub fn desc(&self) -> &str {
        self.errno().desc()
    }

    pub fn code(&self) -> i32 {
        self.errno() as i32
    }
}

#[repr(C)]
#[allow(non_camel_case_types)]
pub enum PrCtlFlag {
    PR_SET_NAME = 15,
}

use mockall::automock;

macro_rules! syscall {
    ($e: expr) => {
        $e.map_err(|e| SysCallError(stringify!($i).to_string(), e))
    };
}

#[automock]
pub mod inner {
    // Bug?: Workaround for mockall code to be detected as dead_code.
    #![allow(dead_code)]

    pub use super::{PrCtlFlag, SysCallError, SysCallResult};
    use crate::status::ExitStatus;
    use nix::{
        errno,
        fcntl::{self, FcntlArg, OFlag},
        libc::{self, c_int, c_ulong},
        sys::{
            signal::{self, SigAction, Signal},
            stat::Mode,
            termios,
            wait::{self, WaitPidFlag, WaitStatus},
        },
        unistd::{self, ForkResult, Pid},
    };
    use std::{
        collections::HashMap, convert::Infallible, env, ffi::CString, os::unix::io::RawFd,
        path::PathBuf, process,
    };

    pub fn close(fd: RawFd) -> SysCallResult<()> {
        syscall!(unistd::close(fd))
    }

    pub fn dup2(oldfd: RawFd, newfd: RawFd) -> SysCallResult<RawFd> {
        syscall!(unistd::dup2(oldfd, newfd))
    }

    pub fn dup_fd(src: RawFd, dest: RawFd) -> SysCallResult<RawFd> {
        let arg = FcntlArg::F_DUPFD_CLOEXEC(dest);
        syscall!(fcntl::fcntl(src, arg))
    }

    pub fn env_get(key: &str) -> Result<String, env::VarError> {
        env::var(key)
    }

    pub fn env_set(key: &str, value: &str) {
        env::set_var(key, value)
    }

    pub fn env_unset(key: &str) {
        env::remove_var(key)
    }

    pub fn env_vars() -> HashMap<String, String> {
        env::vars().collect::<HashMap<_, _>>()
    }

    pub fn execve(path: CString, args: &[CString], env: &[CString]) -> SysCallResult<Infallible> {
        syscall!(unistd::execve(&path, args, env))
    }

    pub fn exit(code: i32) -> ExitStatus {
        process::exit(code)
    }

    pub fn fork() -> SysCallResult<ForkResult> {
        unsafe { syscall!(unistd::fork()) }
    }

    pub fn getpgid(pid: Option<Pid>) -> SysCallResult<Pid> {
        syscall!(unistd::getpgid(pid))
    }

    pub fn getpid() -> Pid {
        unistd::getpid()
    }

    pub fn isatty(fd: RawFd) -> SysCallResult<bool> {
        syscall!(unistd::isatty(fd))
    }

    pub fn open(path: &str, oflag: OFlag, mode: Mode) -> SysCallResult<RawFd> {
        syscall!(fcntl::open(path, oflag, mode))
    }

    pub fn pipe() -> SysCallResult<(RawFd, RawFd)> {
        syscall!(unistd::pipe2(OFlag::O_CLOEXEC))
    }

    pub fn prctl(flag: PrCtlFlag, arg1: c_ulong) -> SysCallResult<()> {
        match unsafe { libc::prctl(flag as c_int, arg1) } {
            0 => Ok(()),
            -1 => {
                let e = SysCallError::new("prctl", errno::from_i32(errno::errno()));
                Err(e)
            }
            _ => unreachable![],
        }
    }

    pub fn read(fd: RawFd, buf: &mut [u8]) -> SysCallResult<usize> {
        syscall!(unistd::read(fd, buf))
    }

    pub fn set_current_dir(path: PathBuf) -> Result<(), std::io::Error> {
        env::set_current_dir(path)
    }

    pub fn setpgid(pid: Pid, pgid: Pid) -> SysCallResult<()> {
        syscall!(unistd::setpgid(pid, pgid))
    }

    pub fn sigaction(sig: Signal, act: &SigAction) -> SysCallResult<SigAction> {
        unsafe { syscall!(signal::sigaction(sig, act)) }
    }

    pub fn tcgetsid(fd: RawFd) -> SysCallResult<Pid> {
        syscall!(termios::tcgetsid(fd))
    }

    pub fn tcgetpgrp(fd: RawFd) -> SysCallResult<Pid> {
        syscall!(unistd::tcgetpgrp(fd))
    }

    pub fn tcsetpgrp(fd: RawFd, pgrp: Pid) -> SysCallResult<()> {
        syscall!(unistd::tcsetpgrp(fd, pgrp))
    }

    pub fn waitpid(pid: Pid, options: Option<WaitPidFlag>) -> SysCallResult<WaitStatus> {
        syscall!(wait::waitpid(pid, options))
    }
}
