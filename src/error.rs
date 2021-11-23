use super::{
    exec::SysCallError,
    location::{Annotate, Location},
    parser::token::{Token, TokenKind},
};
use std::str::Utf8Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellErrorKind {
    AmbiguousRedirect,
    Eof,
    InvalidFd(String),
    InvalidIdentifier(String),
    InvalidUtf8Sequence(Utf8Error),
    SysCallError(String, nix::Error),
    UnexpectedToken(TokenKind),
    Unimplemented(TokenKind),
}
pub type ShellError = Annotate<ShellErrorKind>;

impl ShellError {
    pub fn ambiguous_redirect(loc: Location) -> Self {
        Self::new(ShellErrorKind::AmbiguousRedirect, loc)
    }

    pub fn eof(loc: Location) -> Self {
        Self::new(ShellErrorKind::Eof, loc)
    }

    pub fn invalid_fd(s: &str, loc: Location) -> Self {
        Self::new(ShellErrorKind::InvalidFd(s.to_string()), loc)
    }

    pub fn invalid_identifier(s: String, loc: Location) -> Self {
        Self::new(ShellErrorKind::InvalidIdentifier(s), loc)
    }

    pub fn invalid_utf8_sequence(err: Utf8Error, loc: Location) -> Self {
        Self::new(ShellErrorKind::InvalidUtf8Sequence(err), loc)
    }

    pub fn syscall_error(e: SysCallError, loc: Location) -> Self {
        Self::new(
            ShellErrorKind::SysCallError(e.name().to_string(), e.errno()),
            loc,
        )
    }

    pub fn unexpected_token(t: Token) -> Self {
        Self::new(ShellErrorKind::UnexpectedToken(t.value()), t.location())
    }

    pub fn unimplemented(t: Token) -> Self {
        Self::new(ShellErrorKind::Unimplemented(t.value()), t.location())
    }
}
