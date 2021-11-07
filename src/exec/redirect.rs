use super::syscall::{SysCallResult, SysCallWrapper};
use super::WordParser;
use crate::{
    context::Context,
    error::ShellError,
    location::Location,
    parser::{
        redirect::{Redirect, RedirectKind, RedirectList},
        word::WordList,
    },
    status::Result,
};
use nix::{fcntl::OFlag, sys::stat::Mode};
use std::collections::HashSet;
use std::os::unix::io::RawFd;

pub trait ApplyRedirect {
    fn apply(self, context: &Context, save: bool) -> Result<RedirectList>;
}

impl ApplyRedirect for RedirectList {
    fn apply(self, ctx: &Context, save: bool) -> Result<RedirectList> {
        RedirectApplier::new(save).exec(ctx, self)
    }
}

pub const SHELL_FDBASE: RawFd = 10;

struct RedirectApplier {
    save: bool,
    savefd: [Option<RawFd>; 3], // fd <= 2
    openfd: HashSet<RawFd>,     // fd > 2
}

impl RedirectApplier {
    fn new(save: bool) -> Self {
        Self {
            save,
            savefd: [None, None, None],
            openfd: HashSet::new(),
        }
    }

    fn exec(&mut self, ctx: &Context, list: RedirectList) -> Result<RedirectList> {
        for redirect in list {
            let (kind, loc) = redirect.take();

            let flag_read = OFlag::O_RDONLY;
            let flag_write = OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC;
            let flag_append = OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_APPEND;
            let flag_rw = OFlag::O_RDWR | OFlag::O_CREAT;

            match kind {
                RedirectKind::ReadFrom(fd, wordlist) => self.open(ctx, fd, wordlist, flag_read),
                RedirectKind::WriteTo(fd, wordlist, _force) => {
                    self.open(ctx, fd, wordlist, flag_write)
                }
                RedirectKind::WriteBoth(wordlist) => self
                    .open(ctx, 1, wordlist, flag_write)
                    .and_then(|_| self.copy(ctx, 1, 2)),
                RedirectKind::Copy(src, dest, close) => {
                    self.copy(ctx, src, dest).and_then(|_| match close {
                        false => Ok(()),
                        true => self.close(ctx, src),
                    })
                }
                RedirectKind::Append(fd, wordlist) => self.open(ctx, fd, wordlist, flag_append),
                RedirectKind::AppendBoth(wordlist) => self
                    .open(ctx, 1, wordlist, flag_append)
                    .and_then(|_| self.copy(ctx, 1, 2)),
                RedirectKind::Close(fd) => self.close(ctx, fd),
                RedirectKind::ReadWrite(fd, wordlist) => self.open(ctx, fd, wordlist, flag_rw),
            }
            .map_err(|e| {
                self.save = false;
                self.exec(ctx, self.restore_list()).ok();
                ShellError::syscall_error(e, loc)
            })?;
        }

        Ok(self.restore_list())
    }

    fn open(
        &mut self,
        ctx: &Context,
        fd: RawFd,
        wordlist: WordList,
        flag: OFlag,
    ) -> SysCallResult<()> {
        let file = wordlist.to_string(ctx);
        let mode = Mode::from_bits(0o666).unwrap();
        let new_fd = ctx.wrapper().open(&*file, flag, mode)?;

        if fd == new_fd {
            self.openfd.insert(fd);
        } else {
            self.copy(ctx, new_fd, fd)?;
            self.close(ctx, new_fd)?;
        }

        Ok(())
    }

    fn copy(&mut self, ctx: &Context, src: RawFd, dest: RawFd) -> SysCallResult<()> {
        match dest {
            fd if self.save && fd <= 2 && ctx.wrapper().isatty(dest).unwrap_or(false) => {
                self.savefd[dest as usize] = match self.savefd[dest as usize] {
                    Some(f) => Some(f),
                    None => Some(ctx.wrapper().dup_fd(dest, SHELL_FDBASE)?),
                }
            }
            fd if fd >= 3 => {
                self.openfd.insert(fd);
            }
            _ => (),
        }

        ctx.wrapper().dup2(src, dest)?;
        Ok(())
    }

    fn close(&mut self, ctx: &Context, fd: RawFd) -> SysCallResult<()> {
        if self.save && fd <= 2 && ctx.wrapper().isatty(fd).unwrap_or(false) {
            self.savefd[fd as usize] = match self.savefd[fd as usize] {
                Some(f) => Some(f),
                None => Some(ctx.wrapper().dup_fd(fd, SHELL_FDBASE)?),
            }
        }
        ctx.wrapper().close(fd).unwrap();
        Ok(())
    }

    fn restore_list(&self) -> RedirectList {
        vec![
            self.savefd
                .iter()
                .enumerate()
                .filter(|(_, s)| s.is_some())
                .map(|(d, s)| Redirect::copy(s.unwrap(), d as RawFd, true, Location::new(0, 0)))
                .collect::<Vec<_>>(),
            self.openfd
                .iter()
                .map(|fd| Redirect::close(*fd, Location::new(0, 0)))
                .collect::<Vec<_>>(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
    }
}

include!("redirect_test.rs");
