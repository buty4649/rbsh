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
}

mod mockable {
    use super::{SysCallError, SysCallResult};
    use crate::{parser::redirect::FdSize, status::ExitStatus};
    use nix::{
        fcntl::{open, OFlag},
        sys::stat::Mode,
        sys::wait::waitpid,
        sys::wait::{WaitPidFlag, WaitStatus},
        unistd::{close, dup2, execve, fork, ForkResult, Pid},
    };
    use std::{convert::Infallible, ffi::CString, process::exit};

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

        fn open(&self, path: &str, oflag: OFlag, mode: Mode) -> SysCallResult<FdSize> {
            syscall!(open, path, oflag, mode)
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
    } else {
        pub struct Wrapper {}
        impl Wrapper {
            pub fn new() -> Self {
                Self{}
            }
        }
        impl SysCallWrapper for Wrapper {}
    }
}
