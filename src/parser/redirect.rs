use super::{
    parse_wordlist,
    token::{TokenKind, TokenReader},
    WordList,
};
use crate::{
    error::ShellError,
    location::{Annotate, Location},
    status::Result,
};
use std::os::unix::io::RawFd;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RedirectKind {
    ReadFrom(RawFd, WordList),      // fd filename / n<word
    WriteTo(RawFd, WordList, bool), // fd filename force / n>word
    WriteBoth(WordList),             // filename / &>word, >&word
    Copy(RawFd, RawFd, bool),      // fd(src) fd(dest) close? / n<&n, n<&n-
    Append(RawFd, WordList),        // fd filename / n>>word
    AppendBoth(WordList),            // fd filename / &>>word
    Close(RawFd),                   // fd / n<&-, n>&-
    ReadWrite(RawFd, WordList),     // fd filename / n<>word
}
pub type Redirect = Annotate<RedirectKind>;
pub type RedirectList = Vec<Redirect>;

impl Redirect {
    pub fn read_from(fd: RawFd, wordlist: WordList, loc: Location) -> Self {
        Self::new(RedirectKind::ReadFrom(fd, wordlist), loc)
    }

    pub fn write_to(fd: RawFd, wordlist: WordList, force: bool, loc: Location) -> Self {
        Self::new(RedirectKind::WriteTo(fd, wordlist, force), loc)
    }

    pub fn write_both(wordlist: WordList, loc: Location) -> Self {
        Self::new(RedirectKind::WriteBoth(wordlist), loc)
    }

    pub fn copy(src: RawFd, dest: RawFd, close: bool, loc: Location) -> Self {
        Self::new(RedirectKind::Copy(src, dest, close), loc)
    }

    pub fn append(fd: RawFd, wordlist: WordList, loc: Location) -> Self {
        Self::new(RedirectKind::Append(fd, wordlist), loc)
    }

    pub fn append_both(wordlist: WordList, loc: Location) -> Self {
        Self::new(RedirectKind::AppendBoth(wordlist), loc)
    }

    pub fn close(fd: RawFd, loc: Location) -> Self {
        Self::new(RedirectKind::Close(fd), loc)
    }

    pub fn read_write(fd: RawFd, wordlist: WordList, loc: Location) -> Self {
        Self::new(RedirectKind::ReadWrite(fd, wordlist), loc)
    }
}

pub fn parse_redirect(tokens: &mut TokenReader) -> Result<Option<Redirect>> {
    let loc = tokens.location();

    // parse destination fd
    let fd = match tokens.peek_token() {
        Some(TokenKind::Number(fd)) => {
            let fd = fd
                .parse::<RawFd>()
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
            Err(ShellError::unimplemented(tokens.next().unwrap()))
        }
        Some(TokenKind::HereString) => {
            // <<: Here String
            Err(ShellError::unimplemented(tokens.next().unwrap()))
        }
        _ => return Ok(None),
    }?;

    let redirect = Redirect::new(redirect, loc);
    Ok(Some(redirect))
}

fn parse_redirect_read_from(tokens: &mut TokenReader, fd: RawFd) -> Result<RedirectKind> {
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
    fd: RawFd,
    force: bool,
) -> Result<RedirectKind> {
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

fn parse_redirect_write_both(tokens: &mut TokenReader) -> Result<RedirectKind> {
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

fn parse_redirect_copy(tokens: &mut TokenReader, dest: RawFd) -> Result<RedirectKind> {
    tokens.next();
    match tokens.peek_token() {
        Some(TokenKind::Number(src)) => {
            let src = src
                .parse::<RawFd>()
                .map_err(|_| tokens.error_invalid_fd(&src))?;
            tokens.next();
            let close = match tokens.peek_token() {
                Some(TokenKind::Hyphen) => true,
                _ => false,
            };
            let redirect = RedirectKind::Copy(src, dest, close);
            Ok(redirect)
        }
        _ => Err(tokens.error_unexpected_token()),
    }
}

fn parse_redirect_append(tokens: &mut TokenReader, fd: RawFd) -> Result<RedirectKind> {
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

fn parse_redirect_append_both(tokens: &mut TokenReader) -> Result<RedirectKind> {
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

fn parse_redirect_close(tokens: &mut TokenReader, fd: RawFd) -> Result<RedirectKind> {
    tokens.next();
    Ok(RedirectKind::Close(fd))
}

fn parse_redirect_read_write(tokens: &mut TokenReader, fd: RawFd) -> Result<RedirectKind> {
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
