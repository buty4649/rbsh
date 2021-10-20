mod mockable {
    use crate::parser::redirect::FdSize;
    use nix::{
        fcntl::{open, OFlag},
        sys::stat::Mode,
        unistd::{close, dup2},
    };

    #[cfg(test)]
    use mockall::automock;

    #[cfg_attr(test, automock)]
    pub trait SysCallWrapper {
        fn close(&self, fd: FdSize) -> nix::Result<()> {
            close(fd)
        }

        fn dup2(&self, oldfd: FdSize, newfd: FdSize) -> nix::Result<FdSize> {
            dup2(oldfd, newfd)
        }

        fn open(&self, path: &str, oflag: OFlag, mode: Mode) -> nix::Result<FdSize> {
            open(path, oflag, mode)
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
