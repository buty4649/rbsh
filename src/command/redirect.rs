use super::WordParser;
use crate::parser::{
    redirect::{FdSize, RedirectKind, RedirectList},
    word::WordList,
};
use nix::{
    fcntl::{open, OFlag},
    sys::stat::Mode,
    unistd::{close, dup2},
};

pub trait ApplyRedirect {
    fn apply(self);
}

impl ApplyRedirect for RedirectList {
    fn apply(self) {
        RedirectApplier::new(Box::new(Wrapper {})).exec(self)
    }
}

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
trait FsWrapper {
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

struct Wrapper {}
impl FsWrapper for Wrapper {}

struct RedirectApplier<'a> {
    wrapper: Box<dyn FsWrapper + 'a>,
}

impl<'a> RedirectApplier<'a> {
    fn new(wrapper: Box<dyn FsWrapper + 'a>) -> Self {
        Self { wrapper }
    }

    fn exec(self, list: RedirectList) {
        list.into_iter().for_each(|redirect| {
            let (redirect, _) = redirect.take();

            match redirect {
                RedirectKind::ReadFrom(fd, wordlist) => self.read_from(fd, wordlist),
                RedirectKind::WriteTo(fd, wordlist, _) => self.write_to(fd, wordlist),
                RedirectKind::WriteBoth(wordlist) => {
                    self.write_to(1, wordlist);
                    self.copy(1, 2);
                }
                RedirectKind::Copy(src, dest, close) => {
                    self.copy(src, dest);
                    if close {
                        self.close(src);
                    }
                }
                RedirectKind::Append(fd, wordlist) => self.append(fd, wordlist),
                RedirectKind::AppendBoth(wordlist) => {
                    self.append(1, wordlist);
                    self.copy(1, 2);
                }
                RedirectKind::Close(fd) => self.close(fd),
                RedirectKind::ReadWrite(fd, wordlist) => self.read_write(fd, wordlist),
            }
        })
    }

    fn read_from(&self, fd: FdSize, file: WordList) {
        let file = file.to_string();
        let flag = OFlag::O_RDONLY;
        let mode = Mode::from_bits(0o666).unwrap();
        let new_fd = self.wrapper.open(&*file, flag, mode).unwrap();
        if fd != new_fd {
            self.copy(new_fd, fd);
            self.close(new_fd);
        }
    }

    fn write_to(&self, fd: FdSize, file: WordList) {
        let file = file.to_string();
        let flag = OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC;
        let mode = Mode::from_bits(0o666).unwrap();
        let new_fd = self.wrapper.open(&*file, flag, mode).unwrap();
        if fd != new_fd {
            self.copy(new_fd, fd);
            self.close(new_fd);
        }
    }

    fn copy(&self, src: FdSize, dest: FdSize) {
        self.wrapper.dup2(src, dest).unwrap();
    }

    fn append(&self, fd: FdSize, file: WordList) {
        let file = file.to_string();
        let flag = OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_APPEND;
        let mode = Mode::from_bits(0o666).unwrap();
        let new_fd = self.wrapper.open(&*file, flag, mode).unwrap();
        if fd != new_fd {
            self.copy(new_fd, fd);
            self.close(new_fd);
        }
    }

    fn close(&self, fd: FdSize) {
        self.wrapper.close(fd).unwrap();
    }

    fn read_write(&self, fd: FdSize, file: WordList) {
        let file = file.to_string();
        let flag = OFlag::O_RDWR | OFlag::O_CREAT;
        let mode = Mode::from_bits(0o666).unwrap();
        let new_fd = self.wrapper.open(&*file, flag, mode).unwrap();
        if fd != new_fd {
            self.copy(new_fd, fd);
            self.close(new_fd);
        }
    }
}

include!("redirect_test.rs");
