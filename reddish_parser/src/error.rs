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
    pub fn eof(loc: Location) -> Self {
        Self::new(ErrorKind::Eof, loc)
    }

    pub fn unterminated_string(loc: Location) -> Self {
        Self::new(ErrorKind::UnterminatedString, loc)
    }

    pub fn unexpected_token(t: &Token) -> Self {
        Self::new(ErrorKind::UnexpectedToken(t.value.clone()), t.location)
    }

    pub fn invalid_fd(s: &str, loc: Location) -> Self {
        Self::new(ErrorKind::InvalidFd(s.to_string()), loc)
    }

    pub fn unimplemented(t: Token) -> Self {
        Self::new(ErrorKind::Unimplemented(t.value), t.location)
    }
}

impl From<&Error> for Error {
    fn from(item: &Error) -> Self {
        item.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::location;

    #[test]
    fn error_eof() {
        let err = Error::eof(location!());
        assert_eq!(err.value, ErrorKind::Eof);
        assert_eq!(err.location, location!());
    }

    #[test]
    fn error_unterminated_string() {
        let err = Error::unterminated_string(location!());
        assert_eq!(err.value, ErrorKind::UnterminatedString);
        assert_eq!(err.location, location!());
    }

    #[test]
    fn error_unexpected_token() {
        let token = Token::space(location!());
        let err = Error::unexpected_token(&token);
        assert_eq!(err.value, ErrorKind::UnexpectedToken(TokenKind::Space));
        assert_eq!(err.location, location!());
    }

    #[test]
    fn error_invalid_fd() {
        let err = Error::invalid_fd("abc", location!());
        assert_eq!(err.value, ErrorKind::InvalidFd("abc".to_string()));
        assert_eq!(err.location, location!());
    }

    #[test]
    fn error_unimplemented() {
        let token = Token::space(location!());
        let err = Error::unimplemented(token);
        assert_eq!(err.value, ErrorKind::Unimplemented(TokenKind::Space));
        assert_eq!(err.location, location!());
    }
}
