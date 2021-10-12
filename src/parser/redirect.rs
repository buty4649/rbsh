use super::{parse_wordlist, Annotate, Location, ParseError, TokenKind, WordList};
use crate::token::TokenReader;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RedirectKind {
    ReadFrom(FdSize, WordList),      // fd filename / n<word
    WriteTo(FdSize, WordList, bool), // fd filename force / n>word
    WriteBoth(WordList),             // filename / &>word, >&word
    ReadCopy(FdSize, FdSize, bool),  // fd(src) fd(dest) close? / n<&n, n<&n-
    WriteCopy(FdSize, FdSize, bool), // fd(src) fd(dest) close? / n>&n, n>&n-
    Append(FdSize, WordList),        // fd filename / n>>word
    AppendBoth(WordList),            // fd filename / &>>word
    Close(FdSize),                   // fd / n<&-, n>&-
    ReadWrite(FdSize, WordList),     // fd filename / n<>word
}
pub type FdSize = u16;
pub type Redirect = Annotate<RedirectKind>;

impl Redirect {
    pub fn read_from(fd: FdSize, wordlist: WordList, loc: Location) -> Self {
        Self::new(RedirectKind::ReadFrom(fd, wordlist), loc)
    }

    pub fn write_to(fd: FdSize, wordlist: WordList, force: bool, loc: Location) -> Self {
        Self::new(RedirectKind::WriteTo(fd, wordlist, force), loc)
    }

    pub fn write_both(wordlist: WordList, loc: Location) -> Self {
        Self::new(RedirectKind::WriteBoth(wordlist), loc)
    }

    pub fn read_copy(src: FdSize, dest: FdSize, close: bool, loc: Location) -> Self {
        Self::new(RedirectKind::ReadCopy(src, dest, close), loc)
    }

    pub fn write_copy(src: FdSize, dest: FdSize, close: bool, loc: Location) -> Self {
        Self::new(RedirectKind::WriteCopy(src, dest, close), loc)
    }

    pub fn append(fd: FdSize, wordlist: WordList, loc: Location) -> Self {
        Self::new(RedirectKind::Append(fd, wordlist), loc)
    }

    pub fn append_both(wordlist: WordList, loc: Location) -> Self {
        Self::new(RedirectKind::AppendBoth(wordlist), loc)
    }

    pub fn close(fd: FdSize, loc: Location) -> Self {
        Self::new(RedirectKind::Close(fd), loc)
    }

    pub fn read_write(fd: FdSize, wordlist: WordList, loc: Location) -> Self {
        Self::new(RedirectKind::ReadWrite(fd, wordlist), loc)
    }
}

pub fn parse_redirect(tokens: &mut TokenReader) -> Result<Option<Redirect>, ParseError> {
    let loc = tokens.location();

    // parse destination fd
    let fd = match tokens.peek_token() {
        Some(TokenKind::Number(fd)) => {
            let fd = fd
                .parse::<FdSize>()
                .map_err(|_| tokens.error_invalid_fd(&fd))?;
            tokens.next();
            Some(fd)
        }
        _ => None,
    };

    let redirect = match tokens.peek_token() {
        Some(TokenKind::ReadFrom) => parse_redirect_read_from(tokens, fd.unwrap_or(0)),
        Some(TokenKind::WriteTo) => parse_redirect_write_to(tokens, fd.unwrap_or(1), false),
        Some(TokenKind::ForceWriteTo) => parse_redirect_write_to(tokens, fd.unwrap_or(1), true),
        Some(TokenKind::WriteBoth) => parse_redirect_write_both(tokens),
        Some(TokenKind::ReadCopy) => parse_redirect_copy(tokens, fd.unwrap_or(0)),
        Some(TokenKind::WriteCopy) => parse_redirect_copy(tokens, fd.unwrap_or(1)),
        Some(TokenKind::Append) => parse_redirect_append(tokens, fd.unwrap_or(1)),
        Some(TokenKind::AppendBoth) => parse_redirect_append_both(tokens),
        Some(TokenKind::ReadClose) => parse_redirect_close(tokens, fd.unwrap_or(0)),
        Some(TokenKind::WriteClose) => parse_redirect_close(tokens, fd.unwrap_or(1)),
        Some(TokenKind::ReadWrite) => parse_redirect_read_write(tokens, fd.unwrap_or(0)),
        Some(TokenKind::HereDocument) => {
            // <<: Here Document
            Err(ParseError::unimplemented(tokens.next().unwrap()))
        }
        Some(TokenKind::HereString) => {
            // <<: Here String
            Err(ParseError::unimplemented(tokens.next().unwrap()))
        }
        _ => return Ok(None),
    }?;

    let redirect = Redirect::new(redirect, loc);
    Ok(Some(redirect))
}

fn parse_redirect_read_from(
    tokens: &mut TokenReader,
    fd: FdSize,
) -> Result<RedirectKind, ParseError> {
    tokens.next();
    tokens.skip_space();
    match tokens.peek_token() {
        Some(TokenKind::Word(_, _)) => {
            let words = parse_wordlist(tokens)?;
            Ok(RedirectKind::ReadFrom(fd, words))
        }
        _ => Err(tokens.error_unexpected_token()),
    }
}

fn parse_redirect_write_to(
    tokens: &mut TokenReader,
    fd: FdSize,
    force: bool,
) -> Result<RedirectKind, ParseError> {
    tokens.next();
    tokens.skip_space();
    match tokens.peek_token() {
        Some(TokenKind::Word(_, _)) => {
            let words = parse_wordlist(tokens)?;
            Ok(RedirectKind::WriteTo(fd, words, force))
        }
        _ => Err(tokens.error_unexpected_token()),
    }
}

fn parse_redirect_write_both(tokens: &mut TokenReader) -> Result<RedirectKind, ParseError> {
    tokens.next();
    tokens.skip_space();
    match tokens.peek_token() {
        Some(TokenKind::Word(_, _)) => {
            tokens.skip_space();
            let words = parse_wordlist(tokens)?;
            Ok(RedirectKind::WriteBoth(words))
        }
        _ => Err(tokens.error_unexpected_token()),
    }
}

fn parse_redirect_copy(tokens: &mut TokenReader, dest: FdSize) -> Result<RedirectKind, ParseError> {
    let kind = tokens.next().unwrap().value;
    match tokens.peek_token() {
        Some(TokenKind::Number(src)) => {
            let src = src
                .parse::<FdSize>()
                .map_err(|_| tokens.error_invalid_fd(&src))?;
            tokens.next();
            let close = match tokens.peek_token() {
                Some(TokenKind::Hyphen) => true,
                _ => false,
            };
            let redirect = match kind {
                TokenKind::ReadCopy => RedirectKind::ReadCopy(src, dest, close),
                TokenKind::WriteCopy => RedirectKind::WriteCopy(src, dest, close),
                _ => unreachable![],
            };
            Ok(redirect)
        }
        _ => Err(tokens.error_unexpected_token()),
    }
}

fn parse_redirect_append(tokens: &mut TokenReader, fd: FdSize) -> Result<RedirectKind, ParseError> {
    tokens.next();
    tokens.skip_space();
    match tokens.peek_token() {
        Some(TokenKind::Word(_, _)) => {
            let words = parse_wordlist(tokens)?;
            Ok(RedirectKind::Append(fd, words))
        }
        _ => Err(tokens.error_unexpected_token()),
    }
}

fn parse_redirect_append_both(tokens: &mut TokenReader) -> Result<RedirectKind, ParseError> {
    tokens.next();
    tokens.skip_space();
    match tokens.peek_token() {
        Some(TokenKind::Word(_, _)) => {
            let words = parse_wordlist(tokens)?;
            Ok(RedirectKind::AppendBoth(words))
        }
        _ => Err(tokens.error_unexpected_token()),
    }
}

fn parse_redirect_close(tokens: &mut TokenReader, fd: FdSize) -> Result<RedirectKind, ParseError> {
    tokens.next();
    Ok(RedirectKind::Close(fd))
}

fn parse_redirect_read_write(
    tokens: &mut TokenReader,
    fd: FdSize,
) -> Result<RedirectKind, ParseError> {
    tokens.next();
    tokens.skip_space();
    match tokens.peek_token() {
        Some(TokenKind::Word(_, _)) => {
            let words = parse_wordlist(tokens)?;
            Ok(RedirectKind::ReadWrite(fd, words))
        }
        _ => Err(tokens.error_unexpected_token()),
    }
}

include!("redirect_test.rs");
