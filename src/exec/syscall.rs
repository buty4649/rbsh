pub type SysCallResult<T> = Result<T, SysCallError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SysCallError(String, nix::errno::Errno);

impl SysCallError {
    pub fn new<T: AsRef<str>>(name: T, errno: nix::errno::Errno) -> Self {
        let name = name.as_ref().to_string();
        Self(name, errno)
    }

    pub fn name(&self) -> &str {
        &*self.0
    }

    pub fn errno(&self) -> nix::errno::Errno {
        self.1
    }

    pub fn desc(&self) -> &str {
        self.errno().desc()
    }
}

mod mockable {
    use super::{SysCallError, SysCallResult};
    use crate::{parser::redirect::FdSize, status::ExitStatus};
    use nix::{
        fcntl::{fcntl, open, FcntlArg, OFlag},
        sys::{
            signal::{sigaction, SigAction, Signal},
            stat::Mode,
            termios::tcgetsid,
            wait::{waitpid, WaitPidFlag, WaitStatus},
        },
        unistd::{
            close, dup2, execve, fork, getpgid, getpid, isatty, pipe2, read, setpgid, tcgetpgrp,
            tcsetpgrp, ForkResult, Pid,
        },
    };
    use std::{
        collections::HashMap, convert::Infallible, env, ffi::CString, os::unix::io::RawFd,
        process::exit,
    };

    #[cfg(test)]
    use mockall::automock;

    macro_rules! syscall {
        ($i: ident $(, $e: expr )*) => {
            $i($($e,)*).map_err(|e| SysCallError(stringify!($i).to_string(), e))
        };
    }

    #[cfg_attr(test, automock)]
    pub trait SysCallWrapper {
        fn close(&self, fd: FdSize) -> SysCallResult<()> {
            syscall!(close, fd)
        }

        fn dup2(&self, oldfd: FdSize, newfd: FdSize) -> SysCallResult<FdSize> {
            syscall!(dup2, oldfd, newfd)
        }

        fn dup_fd(&self, src: FdSize, dest: FdSize) -> SysCallResult<FdSize> {
            let arg = FcntlArg::F_DUPFD_CLOEXEC(dest);
            syscall!(fcntl, src, arg)
        }

        fn env_get(&self, key: &str) -> Result<String, env::VarError> {
            env::var(key)
        }

        fn env_set(&self, key: &str, value: &str) {
            env::set_var(key, value)
        }

        fn env_vars(&self) -> HashMap<String, String> {
            env::vars().collect::<HashMap<_, _>>()
        }

        fn execve(
            &self,
            path: CString,
            args: &Vec<CString>,
            env: &Vec<CString>,
        ) -> SysCallResult<Infallible> {
            syscall!(execve, &path, args, env)
        }

        fn exit(&self, code: i32) -> ExitStatus {
            exit(code);
        }

        fn fork(&self) -> SysCallResult<ForkResult> {
            unsafe { syscall!(fork) }
        }

        fn getpgid(&self, pid: Option<Pid>) -> SysCallResult<Pid> {
            syscall!(getpgid, pid)
        }

        fn getpid(&self) -> Pid {
            getpid()
        }

        fn isatty(&self, fd: RawFd) -> SysCallResult<bool> {
            syscall!(isatty, fd)
        }

        fn open(&self, path: &str, oflag: OFlag, mode: Mode) -> SysCallResult<FdSize> {
            syscall!(open, path, oflag, mode)
        }

        fn pipe(&self) -> SysCallResult<(RawFd, RawFd)> {
            syscall!(pipe2, OFlag::O_CLOEXEC)
        }

        fn read(&self, fd: RawFd, buf: &mut [u8]) -> SysCallResult<usize> {
            syscall!(read, fd, buf)
        }

        fn setpgid(&self, pid: Pid, pgid: Pid) -> SysCallResult<()> {
            syscall!(setpgid, pid, pgid)
        }

        fn sigaction(&self, sig: Signal, act: &SigAction) -> SysCallResult<SigAction> {
            unsafe { syscall!(sigaction, sig, act) }
        }

        fn tcgetsid(&self, fd: RawFd) -> SysCallResult<Pid> {
            syscall!(tcgetsid, fd)
        }

        fn tcgetpgrp(&self, fd: RawFd) -> SysCallResult<Pid> {
            syscall!(tcgetpgrp, fd)
        }

        fn tcsetpgrp(&self, fd: RawFd, pgrp: Pid) -> SysCallResult<()> {
            syscall!(tcsetpgrp, fd, pgrp)
        }

        fn waitpid(&self, pid: Pid, options: Option<WaitPidFlag>) -> SysCallResult<WaitStatus> {
            syscall!(waitpid, pid, options)
        }
    }
}

pub use mockable::SysCallWrapper;

cfg_if::cfg_if! {
    if #[cfg(test)] {
        pub use mockable::MockSysCallWrapper as Wrapper;
        impl Clone for Wrapper {
            fn clone(&self) -> Self {
                Self::new()
            }
        }
    } else {
        #[derive(Debug, Clone)]
        pub struct Wrapper {}
        impl Wrapper {
            pub fn new() -> Self {
                Self{}
            }
        }
        impl SysCallWrapper for Wrapper {}
    }
}
