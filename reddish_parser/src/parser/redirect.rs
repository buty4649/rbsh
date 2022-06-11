use super::{error_unexpected_token, parse_wordlist};
use crate::{lexer::LexerIterator, Annotate, Error, Location, Result, TokenKind, Word};
use std::os::unix::io::RawFd;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RedirectKind {
    ReadFrom(RawFd, Vec<Word>),      // fd filename / n<word
    WriteTo(RawFd, Vec<Word>, bool), // fd filename force / n>word
    WriteBoth(Vec<Word>),            // filename / &>word, >&word
    Copy(RawFd, RawFd, bool),        // fd(src) fd(dest) close? / n<&n, n<&n-
    Append(RawFd, Vec<Word>),        // fd filename / n>>word
    AppendBoth(Vec<Word>),           // fd filename / &>>word
    Close(RawFd),                    // fd / n<&-, n>&-
    ReadWrite(RawFd, Vec<Word>),     // fd filename / n<>word
}

pub type Redirect = Annotate<RedirectKind>;

impl Redirect {
    pub fn read_from(fd: RawFd, words: Vec<Word>, loc: Location) -> Self {
        Self::new(RedirectKind::ReadFrom(fd, words), loc)
    }

    pub fn write_to(fd: RawFd, words: Vec<Word>, force: bool, loc: Location) -> Self {
        Self::new(RedirectKind::WriteTo(fd, words, force), loc)
    }

    pub fn write_both(words: Vec<Word>, loc: Location) -> Self {
        Self::new(RedirectKind::WriteBoth(words), loc)
    }

    pub fn copy(src: RawFd, dest: RawFd, close: bool, loc: Location) -> Self {
        Self::new(RedirectKind::Copy(src, dest, close), loc)
    }

    pub fn append(fd: RawFd, words: Vec<Word>, loc: Location) -> Self {
        Self::new(RedirectKind::Append(fd, words), loc)
    }

    pub fn append_both(words: Vec<Word>, loc: Location) -> Self {
        Self::new(RedirectKind::AppendBoth(words), loc)
    }

    pub fn close(fd: RawFd, loc: Location) -> Self {
        Self::new(RedirectKind::Close(fd), loc)
    }

    pub fn read_write(fd: RawFd, words: Vec<Word>, loc: Location) -> Self {
        Self::new(RedirectKind::ReadWrite(fd, words), loc)
    }
}

pub fn parse_redirect(lexer: &mut LexerIterator) -> Result<Option<Vec<Redirect>>> {
    let mut result = None;

    while let Some(r) = parse_redirect_internal(lexer)? {
        result.get_or_insert(Vec::new()).push(r);
    }

    Ok(result)
}

pub fn parse_redirect_internal(lexer: &mut LexerIterator) -> Result<Option<Redirect>> {
    let location = lexer.location();

    let fd = match lexer.next_if(|kind| matches!(kind, TokenKind::Number { .. })) {
        Some(token) => {
            let fd = match token?.value {
                TokenKind::Number(fd) => fd
                    .parse::<RawFd>()
                    .map_err(|_| Error::invalid_fd(&fd, location))?,
                _ => unreachable![],
            };
            Some(fd)
        }
        None => None,
    };

    let redirect = match lexer.peek() {
        Some(_) => match lexer.next_if(|kind| {
            matches!(
                kind,
                &TokenKind::ReadFrom
                    | &TokenKind::WriteTo
                    | &TokenKind::ForceWriteTo
                    | &TokenKind::WriteBoth
                    | &TokenKind::ReadCopy
                    | &TokenKind::WriteCopy
                    | &TokenKind::Append
                    | &TokenKind::AppendBoth
                    | &TokenKind::ReadClose
                    | &TokenKind::WriteClose
                    | &TokenKind::ReadWrite
                    | &TokenKind::HereDocument
                    | &TokenKind::HereString
            )
        }) {
            Some(Ok(token)) => match token.value {
                TokenKind::ReadFrom => parse_redirect_read_from(lexer, fd.unwrap_or(0)),
                TokenKind::WriteTo => parse_redirect_write_to(lexer, fd.unwrap_or(1), false),
                TokenKind::ForceWriteTo => parse_redirect_write_to(lexer, fd.unwrap_or(1), true),
                TokenKind::WriteBoth => parse_redirect_write_both(lexer),
                TokenKind::ReadCopy => parse_redirect_copy(lexer, fd.unwrap_or(0)),
                TokenKind::WriteCopy => parse_redirect_copy(lexer, fd.unwrap_or(1)),
                TokenKind::Append => parse_redirect_append(lexer, fd.unwrap_or(1)),
                TokenKind::AppendBoth => parse_redirect_append_both(lexer),
                TokenKind::ReadClose => parse_redirect_close(fd.unwrap_or(0)),
                TokenKind::WriteClose => parse_redirect_close(fd.unwrap_or(1)),
                TokenKind::ReadWrite => parse_redirect_read_write(lexer, fd.unwrap_or(0)),
                TokenKind::HereDocument => return Err(Error::unimplemented(token)),
                TokenKind::HereString => return Err(Error::unimplemented(token)),
                _ => unreachable![],
            }?,
            Some(Err(e)) => return Err(e),
            None => return Ok(None),
        },
        None if fd.is_some() => return Err(Error::eof(location)),
        None => return Ok(None),
    };

    let redirect = Redirect::new(redirect, location);
    Ok(Some(redirect))
}

fn parse_redirect_read_from(lexer: &mut LexerIterator, fd: RawFd) -> Result<RedirectKind> {
    parse_redirect_word(lexer).map(|w| RedirectKind::ReadFrom(fd, w))
}

fn parse_redirect_write_to(
    lexer: &mut LexerIterator,
    fd: RawFd,
    force: bool,
) -> Result<RedirectKind> {
    parse_redirect_word(lexer).map(|w| RedirectKind::WriteTo(fd, w, force))
}

fn parse_redirect_write_both(lexer: &mut LexerIterator) -> Result<RedirectKind> {
    parse_redirect_word(lexer).map(RedirectKind::WriteBoth)
}

fn parse_redirect_copy(lexer: &mut LexerIterator, dest: RawFd) -> Result<RedirectKind> {
    let location = lexer.location();
    match lexer.next_if(|kind| matches!(kind, TokenKind::Number { .. })) {
        Some(token) => match token?.value {
            TokenKind::Number(src) => {
                let src = src
                    .parse::<RawFd>()
                    .map_err(|_| Error::invalid_fd(&src, location))?;
                let close = matches!(lexer.next_if(|k| k == &TokenKind::Hyphen), Some(_));
                Ok(RedirectKind::Copy(src, dest, close))
            }
            _ => unreachable![],
        },
        None => match lexer.peek() {
            Some(Ok(token)) => Err(Error::unexpected_token(token)),
            Some(Err(_)) => unreachable![],
            None => Err(Error::eof(lexer.location())),
        },
    }
}

fn parse_redirect_append(lexer: &mut LexerIterator, fd: RawFd) -> Result<RedirectKind> {
    parse_redirect_word(lexer).map(|w| RedirectKind::Append(fd, w))
}

fn parse_redirect_append_both(lexer: &mut LexerIterator) -> Result<RedirectKind> {
    parse_redirect_word(lexer).map(RedirectKind::AppendBoth)
}

fn parse_redirect_close(fd: RawFd) -> Result<RedirectKind> {
    Ok(RedirectKind::Close(fd))
}

fn parse_redirect_read_write(lexer: &mut LexerIterator, fd: RawFd) -> Result<RedirectKind> {
    parse_redirect_word(lexer).map(|w| RedirectKind::ReadWrite(fd, w))
}

fn parse_redirect_word(lexer: &mut LexerIterator) -> Result<Vec<Word>> {
    lexer.skip_if_space()?;
    match parse_wordlist(lexer)? {
        Some(wordlist) => Ok(wordlist),
        None => Err(error_unexpected_token(lexer)),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{error::Error, lexer::Lexer, location, Token, WordKind};

    macro_rules! lex {
        ($e: expr) => {
            Lexer::new($e, 0).iter()
        };
    }

    macro_rules! assert_redirect {
        ($e: expr, $expect: expr) => {
            let got = parse_redirect(&mut lex!($e));
            assert_eq!(got, $expect)
        };
    }

    macro_rules! ok {
        ($i: ident, $($a: expr$(,)?)+) => {
            Ok(Some(vec![Redirect::$i($($a,)*)]))
        };
    }

    #[test]
    fn test_readfrom() {
        assert_redirect!(
            "< foobar",
            ok![
                read_from,
                0,
                vec![Word::normal("foobar", location!(3, 1))],
                location!(1, 1)
            ]
        );

        assert_redirect!(
            "123< foobar",
            ok![
                read_from,
                123,
                vec![Word::normal("foobar", location!(6, 1))],
                location!(1, 1)
            ]
        );

        assert_redirect!(
            "12345678901234567890< foobar",
            Err(Error::invalid_fd("12345678901234567890", location!(1, 1)))
        );
    }

    #[test]
    fn test_parse_redirect_writeto() {
        assert_redirect!(
            "> foobar",
            ok![
                write_to,
                1,
                vec![Word::normal("foobar", location!(3, 1))],
                false,
                location!(1, 1)
            ]
        );

        assert_redirect!(
            "123> foobar",
            ok![
                write_to,
                123,
                vec![Word::normal("foobar", location!(6, 1))],
                false,
                location!(1, 1)
            ]
        );

        assert_redirect!(
            "123>| foobar",
            ok![
                write_to,
                123,
                vec![Word::normal("foobar", location!(7, 1))],
                true,
                location!(1, 1)
            ]
        );
    }

    #[test]
    fn test_close() {
        assert_redirect!("<&-", ok![close, 0, location!(1, 1)]);
        assert_redirect!(">&-", ok![close, 1, location!(1, 1)]);
        assert_redirect!("123<&-", ok![close, 123, location!(1, 1)]);
        assert_redirect!("123>&-", ok![close, 123, location!(1, 1)]);

        assert_redirect!(
            "12345678901234567890<&-",
            Err(Error::invalid_fd("12345678901234567890", location!(1, 1)))
        );

        assert_redirect!(
            "12345678901234567890>&-",
            Err(Error::invalid_fd("12345678901234567890", location!(1, 1)))
        );
    }

    #[test]
    fn test_writeboth() {
        assert_redirect!(
            "&> foobar",
            ok![
                write_both,
                vec![Word::normal("foobar", location!(4, 1))],
                location!(1, 1)
            ]
        );

        assert_redirect!(
            ">& foobar",
            ok![
                write_both,
                vec![Word::normal("foobar", location!(4, 1))],
                location!(1, 1)
            ]
        );

        assert_redirect!(
            ">&&",
            Err(Error::unexpected_token(&Token::background(location!(3, 1))))
        );
    }

    #[test]
    fn test_readcopy() {
        assert_redirect!("<&123", ok![copy, 123, 0, false, location!(1, 1)]);
        assert_redirect!("<&", Err(Error::eof(location!(3, 1))));
        assert_redirect!("<&123-", ok!(copy, 123, 0, true, location!(1, 1)));
        assert_redirect!("123<&456", ok![copy, 456, 123, false, location!(1, 1)]);
        assert_redirect!("123<&456-", ok![copy, 456, 123, true, location!(1, 1)]);

        assert_redirect!(
            "<& foobar",
            Err(Error::unexpected_token(&Token::space(location!(3, 1))))
        );

        assert_redirect!(
            "<&12345678901234567890",
            Err(Error::invalid_fd("12345678901234567890", location!(3, 1)))
        );
    }

    #[test]
    fn test_writecopy() {
        assert_redirect!(">&123", ok![copy, 123, 1, false, location!(1, 1)]);
        assert_redirect!(">&123-", ok![copy, 123, 1, true, location!(1, 1)]);
        assert_redirect!("123>&456", ok![copy, 456, 123, false, location!(1, 1)]);
        assert_redirect!("123>&456-", ok![copy, 456, 123, true, location!(1, 1)]);

        assert_redirect!(
            "123>&foobar",
            Err(Error::unexpected_token(&Token::word(
                "foobar",
                WordKind::Normal,
                location!(6, 1)
            )))
        );

        assert_redirect!(
            ">&12345678901234567890",
            Err(Error::invalid_fd("12345678901234567890", location!(3, 1)))
        );
    }

    #[test]
    fn test_append() {
        assert_redirect!(
            ">> foobar",
            ok![
                append,
                1,
                vec![Word::normal("foobar", location!(4, 1))],
                location!(1, 1)
            ]
        );

        assert_redirect!(
            "123>> foobar",
            ok![
                append,
                123,
                vec![Word::normal("foobar", location!(7, 1))],
                location!(1, 1)
            ]
        );

        assert_redirect!(
            "12345678901234567890>> foobar",
            Err(Error::invalid_fd("12345678901234567890", location!(1, 1)))
        );
    }

    #[test]
    fn test_append_both() {
        assert_redirect!(
            "&>> foobar",
            ok![
                append_both,
                vec![Word::normal("foobar", location!(5, 1))],
                location!(1, 1)
            ]
        );

        assert_redirect!(
            "&>>&",
            Err(Error::unexpected_token(&Token::background(location!(4, 1))))
        );
    }

    #[test]
    fn test_readwrite() {
        assert_redirect!(
            "<> foobar",
            ok![
                read_write,
                0,
                vec![Word::normal("foobar", location!(4, 1))],
                location!(1, 1)
            ]
        );

        assert_redirect!(
            "123<> foobar",
            ok![
                read_write,
                123,
                vec![Word::normal("foobar", location!(7, 1))],
                location!(1, 1)
            ]
        );

        assert_redirect!(
            "<>&",
            Err(Error::unexpected_token(&Token::background(location!(3, 1))))
        );

        assert_redirect!(
            "12345678901234567890<> foobar",
            Err(Error::invalid_fd("12345678901234567890", location!(1, 1)))
        );
    }
}
