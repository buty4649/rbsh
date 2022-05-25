use crate::{
    location::{Annotate, Location},
    parser::{Token, TokenKind},
};
use std::str::Utf8Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    Eof,
    UnexpectedToken(TokenKind),
    InvalidUtf8Sequence(Utf8Error),
    InvalidFd(String),
    Unimplemented(TokenKind),
}
pub type Error = Annotate<ErrorKind>;

impl Error {
    pub fn eof(loc: Location) -> Self {
        Self::new(ErrorKind::Eof, loc)
    }

    pub fn unexpected_token(t: Token) -> Self {
        Self::new(ErrorKind::UnexpectedToken(t.value()), t.location())
    }

    pub fn invalid_utf8_sequence(err: Utf8Error, loc: Location) -> Self {
        Self::new(ErrorKind::InvalidUtf8Sequence(err), loc)
    }

    pub fn invalid_fd(s: &str, loc: Location) -> Self {
        Self::new(ErrorKind::InvalidFd(s.to_string()), loc)
    }

    pub fn unimplemented(t: Token) -> Self {
        Self::new(ErrorKind::Unimplemented(t.value()), t.location())
    }
}
