use crate::{
    location::{Annotate, Location},
    Token, TokenKind,
};
use std::{convert::From, str::Utf8Error};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    Eof,
    UnterminatedString,
    UnexpectedToken(TokenKind),
    InvalidUtf8Sequence(Utf8Error),
    InvalidFd(String),
    Unimplemented(TokenKind),
}
pub type Error = Annotate<ErrorKind>;

impl Error {
    pub(crate) fn eof(loc: Location) -> Self {
        Self::new(ErrorKind::Eof, loc)
    }

    pub(crate) fn unterminated_string(loc: Location) -> Self {
        Self::new(ErrorKind::UnterminatedString, loc)
    }

    pub(crate) fn unexpected_token(t: &Token) -> Self {
        Self::new(ErrorKind::UnexpectedToken(t.value.clone()), t.location)
    }

    pub(crate) fn invalid_fd(s: &str, loc: Location) -> Self {
        Self::new(ErrorKind::InvalidFd(s.to_string()), loc)
    }

    pub(crate) fn unimplemented(t: Token) -> Self {
        Self::new(ErrorKind::Unimplemented(t.value), t.location)
    }
}

impl From<&Error> for Error {
    fn from(item: &Error) -> Self {
        item.clone()
    }
}
