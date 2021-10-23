use super::syscall::{SysCallWrapper, Wrapper};
use super::WordParser;
use crate::{
    context::Context,
    error::ShellError,
    location::Location,
    parser::{
        redirect::{FdSize, RedirectKind, RedirectList},
        word::WordList,
    },
    status::Result,
};
use nix::{fcntl::OFlag, sys::stat::Mode};

pub trait ApplyRedirect {
    fn apply(self, context: &Context) -> Result<()>;
}

impl ApplyRedirect for RedirectList {
    fn apply(self, context: &Context) -> Result<()> {
        RedirectApplier::new(Wrapper::new(), context).exec(self)
    }
}

struct RedirectApplier<'a> {
    wrapper: Wrapper,
    context: &'a Context,
}
type SysCallResult = super::syscall::SysCallResult<()>;

impl<'a> RedirectApplier<'a> {
    fn new(wrapper: Wrapper, context: &'a Context) -> Self {
        Self { wrapper, context }
    }

    fn exec(self, list: RedirectList) -> Result<()> {
        for redirect in list.iter() {
            let (redirect, _) = redirect.take();

            match redirect {
                RedirectKind::ReadFrom(fd, wordlist) => self.read_from(fd, wordlist),
                RedirectKind::WriteTo(fd, wordlist, _) => self.write_to(fd, wordlist),
                RedirectKind::WriteBoth(wordlist) => {
                    self.write_to(1, wordlist).and_then(|_| self.copy(1, 2))
                }
                RedirectKind::Copy(src, dest, close) => {
                    self.copy(src, dest)
                        .and_then(|r| if close { self.close(src) } else { Ok(r) })
                }
                RedirectKind::Append(fd, wordlist) => self.append(fd, wordlist),
                RedirectKind::AppendBoth(wordlist) => {
                    self.append(1, wordlist).and_then(|_| self.copy(1, 2))
                }
                RedirectKind::Close(fd) => self.close(fd),
                RedirectKind::ReadWrite(fd, wordlist) => self.read_write(fd, wordlist),
            }
            .map_err(|e| ShellError::syscall_error(e, Location::new(1, 1)))?;
        }
        Ok(())
    }

    fn read_from(&self, fd: FdSize, file: WordList) -> SysCallResult {
        let file = file.to_string(self.context);
        let flag = OFlag::O_RDONLY;
        let mode = Mode::from_bits(0o666).unwrap();
        let new_fd = self.wrapper.open(&*file, flag, mode)?;
        if fd != new_fd {
            self.copy(new_fd, fd)?;
            self.close(new_fd)?;
        }
        Ok(())
    }

    fn write_to(&self, fd: FdSize, file: WordList) -> SysCallResult {
        let file = file.to_string(self.context);
        let flag = OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC;
        let mode = Mode::from_bits(0o666).unwrap();
        let new_fd = self.wrapper.open(&*file, flag, mode)?;
        if fd != new_fd {
            self.copy(new_fd, fd)?;
            self.close(new_fd)?;
        }
        Ok(())
    }

    fn copy(&self, src: FdSize, dest: FdSize) -> SysCallResult {
        self.wrapper.dup2(src, dest)?;
        Ok(())
    }

    fn append(&self, fd: FdSize, file: WordList) -> SysCallResult {
        let file = file.to_string(self.context);
        let flag = OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_APPEND;
        let mode = Mode::from_bits(0o666).unwrap();
        let new_fd = self.wrapper.open(&*file, flag, mode)?;
        if fd != new_fd {
            self.copy(new_fd, fd)?;
            self.close(new_fd)?;
        }
        Ok(())
    }

    fn close(&self, fd: FdSize) -> SysCallResult {
        self.wrapper.close(fd)?;
        Ok(())
    }

    fn read_write(&self, fd: FdSize, file: WordList) -> SysCallResult {
        let file = file.to_string(self.context);
        let flag = OFlag::O_RDWR | OFlag::O_CREAT;
        let mode = Mode::from_bits(0o666).unwrap();
        let new_fd = self.wrapper.open(&*file, flag, mode)?;
        if fd != new_fd {
            self.copy(new_fd, fd)?;
            self.close(new_fd)?;
        }
        Ok(())
    }
}

include!("redirect_test.rs");
