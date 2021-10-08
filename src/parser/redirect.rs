use super::{
    parse_wordlist, peek_token, FdSize, Location, ParseError, RedirectKind, Token, TokenKind,
};
use std::iter::Peekable;

pub fn parse_redirect<T>(tokens: &mut Peekable<T>) -> Result<Option<Token>, ParseError>
where
    T: Iterator<Item = Token> + Clone,
{
    // parse destination fd
    let (fd, loc) = match tokens.peek() {
        Some(token) => {
            let loc = token.loc;
            let fd = match &token.value {
                TokenKind::Number(fd) => {
                    let fd = fd.to_string();
                    let loc = tokens.next().unwrap().loc;
                    Some(
                        fd.parse::<FdSize>()
                            .map_err(|_| ParseError::invalid_fd(fd, loc))?,
                    )
                }
                _ => None,
            };
            (fd, loc)
        }
        None => return Ok(None), // EOF
    };

    let redirect = match peek_token(tokens) {
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
    let token = Token::redirect(redirect, loc);
    Ok(Some(token))
}

fn parse_redirect_read_from<T>(
    tokens: &mut Peekable<T>,
    fd: FdSize,
) -> Result<RedirectKind, ParseError>
where
    T: Iterator<Item = Token>,
{
    let loc = tokens.next().unwrap().loc; // Token::ReadFrom

    match peek_token(tokens) {
        Some(TokenKind::Space) | Some(TokenKind::Word(_, _)) => {
            skip_space(tokens);
            let words = parse_wordlist(tokens)?;
            Ok(RedirectKind::ReadFrom(fd, words))
        }
        None => Err(ParseError::eof(Location::new_from_offset(&loc, 1, 0))),
        _ => Err(ParseError::unexpected_token(tokens.next().unwrap())),
    }
}

fn parse_redirect_write_to<T>(
    tokens: &mut Peekable<T>,
    fd: FdSize,
    force: bool,
) -> Result<RedirectKind, ParseError>
where
    T: Iterator<Item = Token>,
{
    let loc = tokens.next().unwrap().loc; // Token::WriteTo

    match peek_token(tokens) {
        Some(TokenKind::Space) | Some(TokenKind::Word(_, _)) => {
            skip_space(tokens);
            let words = parse_wordlist(tokens)?;
            Ok(RedirectKind::WriteTo(fd, words, force))
        }
        None => Err(ParseError::eof(Location::new_from_offset(&loc, 1, 0))),
        _ => Err(ParseError::unexpected_token(tokens.next().unwrap())),
    }
}

fn parse_redirect_write_both<T>(tokens: &mut Peekable<T>) -> Result<RedirectKind, ParseError>
where
    T: Iterator<Item = Token>,
{
    let loc = tokens.next().unwrap().loc; // Token::WriteBoth

    match peek_token(tokens) {
        Some(TokenKind::Space) | Some(TokenKind::Word(_, _)) => {
            skip_space(tokens);
            let words = parse_wordlist(tokens)?;
            Ok(RedirectKind::WriteBoth(words))
        }
        None => Err(ParseError::eof(Location::new_from_offset(&loc, 1, 0))),
        _ => Err(ParseError::unexpected_token(tokens.next().unwrap())),
    }
}

fn parse_redirect_copy<T>(
    tokens: &mut Peekable<T>,
    dest: FdSize,
) -> Result<RedirectKind, ParseError>
where
    T: Iterator<Item = Token>,
{
    let Token { value: kind, loc } = tokens.next().unwrap();

    match peek_token(tokens) {
        Some(TokenKind::Number(src)) => {
            let src = src.to_string();
            let loc = tokens.next().unwrap().loc;
            let src = src
                .parse::<FdSize>()
                .map_err(|_| ParseError::invalid_fd(src, loc))?;
            let close = match peek_token(tokens) {
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
        None => Err(ParseError::eof(Location::new_from_offset(&loc, 2, 0))),
        _ => Err(ParseError::unexpected_token(tokens.next().unwrap())),
    }
}

fn parse_redirect_append<T>(
    tokens: &mut Peekable<T>,
    fd: FdSize,
) -> Result<RedirectKind, ParseError>
where
    T: Iterator<Item = Token>,
{
    let loc = tokens.next().unwrap().loc; // Token::Append

    match peek_token(tokens) {
        Some(TokenKind::Space) | Some(TokenKind::Word(_, _)) => {
            skip_space(tokens);
            let words = parse_wordlist(tokens)?;
            Ok(RedirectKind::Append(fd, words))
        }
        None => Err(ParseError::eof(Location::new_from_offset(&loc, 1, 0))),
        _ => Err(ParseError::unexpected_token(tokens.next().unwrap())),
    }
}

fn parse_redirect_append_both<T>(tokens: &mut Peekable<T>) -> Result<RedirectKind, ParseError>
where
    T: Iterator<Item = Token>,
{
    let loc = tokens.next().unwrap().loc; // Token::Append

    match peek_token(tokens) {
        Some(TokenKind::Space) | Some(TokenKind::Word(_, _)) => {
            skip_space(tokens);
            let words = parse_wordlist(tokens)?;
            Ok(RedirectKind::AppendBoth(words))
        }
        None => Err(ParseError::eof(Location::new_from_offset(&loc, 1, 0))),
        _ => Err(ParseError::unexpected_token(tokens.next().unwrap())),
    }
}

fn parse_redirect_close<T>(tokens: &mut Peekable<T>, fd: FdSize) -> Result<RedirectKind, ParseError>
where
    T: Iterator<Item = Token>,
{
    tokens.next(); // Token::ReadClose or Token::WriteClose
    Ok(RedirectKind::Close(fd))
}

fn parse_redirect_read_write<T>(
    tokens: &mut Peekable<T>,
    fd: FdSize,
) -> Result<RedirectKind, ParseError>
where
    T: Iterator<Item = Token>,
{
    let loc = tokens.next().unwrap().loc; // Token::ReadWrite

    match peek_token(tokens) {
        Some(TokenKind::Space) | Some(TokenKind::Word(_, _)) => {
            skip_space(tokens);
            let words = parse_wordlist(tokens)?;
            Ok(RedirectKind::ReadWrite(fd, words))
        }
        None => Err(ParseError::eof(Location::new_from_offset(&loc, 1, 0))),
        _ => Err(ParseError::unexpected_token(tokens.next().unwrap())),
    }
}

pub fn skip_space<T>(tokens: &mut Peekable<T>) -> Option<Token>
where
    T: Iterator<Item = Token>,
{
    match peek_token(tokens) {
        Some(TokenKind::Space) => tokens.next(),
        _ => None,
    }
}

include!("redirect_test.rs");
