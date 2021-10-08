mod redirect;

use crate::lexer::lex;
use crate::token::{Token, TokenKind};
use redirect::parse_redirect;
use std::iter::Peekable;
use std::str::Utf8Error;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnitKind {
    SimpleCommand {
        command: Vec<Vec<Token>>,
        redirect: Vec<Token>,
        background: bool,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Location {
    column: usize,
    line: usize,
}

impl Location {
    pub fn new(column: usize, line: usize) -> Self {
        Self { column, line }
    }

    pub fn new_from(other: &Self) -> Self {
        Self::new_from_offset(other, 0, 0)
    }

    pub fn new_from_offset(other: &Self, column_offset: usize, line_offset: usize) -> Self {
        Self::new(other.column + column_offset, other.line + line_offset)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Annotate<T> {
    pub value: T,
    loc: Location,
}

impl<T> Annotate<T> {
    pub fn new(value: T, loc: Location) -> Self {
        Self { value, loc }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
    UnexpectedToken(TokenKind),
    UnknownType(char),
    InvalidFd(String),
    AmbiguousRedirect,
    Unimplemented(TokenKind),
    InvalidUtf8Sequence(Utf8Error),
    Eof,
}
pub type ParseError = Annotate<ParseErrorKind>;

impl ParseError {
    pub fn unexpected_token(t: Token) -> Self {
        Self::new(ParseErrorKind::UnexpectedToken(t.value), t.loc)
    }

    pub fn unknown_type(c: char, loc: Location) -> Self {
        Self::new(ParseErrorKind::UnknownType(c), loc)
    }

    pub fn invalid_fd(s: String, loc: Location) -> Self {
        Self::new(ParseErrorKind::InvalidFd(s), loc)
    }

    pub fn ambiguous_redirect(loc: Location) -> Self {
        Self::new(ParseErrorKind::AmbiguousRedirect, loc)
    }

    pub fn unimplemented(t: Token) -> Self {
        Self::new(ParseErrorKind::Unimplemented(t.value), t.loc)
    }

    pub fn invalid_utf8_sequence(err: Utf8Error, loc: Location) -> Self {
        Self::new(ParseErrorKind::InvalidUtf8Sequence(err), loc)
    }

    pub fn eof(loc: Location) -> Self {
        Self::new(ParseErrorKind::Eof, loc)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WordKind {
    Normal,    // word
    Quote,     // "word"
    Literal,   // 'word'
    Command,   // `word`
    Variable,  // $word
    Parameter, // ${word}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RedirectKind {
    ReadFrom(FdSize, Vec<Token>),      // fd filename / n<word
    WriteTo(FdSize, Vec<Token>, bool), // fd filename force / n>word
    WriteBoth(Vec<Token>),             // filename / &>word, >&word
    ReadCopy(FdSize, FdSize, bool),    // fd(src) fd(dest) close? / n<&n, n<&n-
    WriteCopy(FdSize, FdSize, bool),   // fd(src) fd(dest) close? / n>&n, n>&n-
    Append(FdSize, Vec<Token>),        // fd filename / n>>word
    AppendBoth(Vec<Token>),            // fd filename / &>>word
    Close(FdSize),                     // fd / n<&-, n>&-
    ReadWrite(FdSize, Vec<Token>),     // fd filename / n<>word
}
pub type FdSize = u16;

pub fn parse_command_line(s: &str) -> Result<Option<UnitKind>, ParseError> {
    let tokens = lex(s)?;
    let mut tokens = tokens.into_iter().peekable();
    parse_simple_command(&mut tokens)
}

fn parse_simple_command<T>(tokens: &mut Peekable<T>) -> Result<Option<UnitKind>, ParseError>
where
    T: Iterator<Item = Token> + Clone,
{
    let mut command = vec![];
    let mut redirect = vec![];

    loop {
        match parse_redirect(tokens)? {
            None => (),
            Some(r) => {
                redirect.push(r);
                continue;
            }
        }

        match peek_token(tokens) {
            Some(TokenKind::Space) => {
                tokens.next();
            }
            Some(TokenKind::Word(_, _)) => {
                let words = parse_wordlist(tokens)?;
                command.push(words);
            }
            _ => break,
        }
    }

    if command.is_empty() && redirect.is_empty() {
        Ok(None)
    } else {
        Ok(Some(UnitKind::SimpleCommand {
            command,
            redirect,
            background: false,
        }))
    }
}

fn parse_wordlist<T>(tokens: &mut Peekable<T>) -> Result<Vec<Token>, ParseError>
where
    T: Iterator<Item = Token>,
{
    let mut result = vec![];

    loop {
        match peek_token(tokens) {
            Some(TokenKind::Word(_, _)) => {
                let token = tokens.next().unwrap();
                result.push(token)
            }
            Some(TokenKind::Space) => {
                tokens.next();
                break;
            }
            _ => break,
        }
    }
    Ok(result)
}

fn peek_token<T>(tokens: &mut Peekable<T>) -> Option<&TokenKind>
where
    T: Iterator<Item = Token>,
{
    tokens.peek().map(|tok| &tok.value)
}
