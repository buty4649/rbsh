pub mod command;
pub mod redirect;
pub mod word;

use crate::lexer::lex;
use crate::token::{Token, TokenKind, TokenReader};
use command::{parse_command, ConnecterKind};
use redirect::Redirect;
use std::iter::Iterator;
use std::str::Utf8Error;
use word::{parse_wordlist, WordList};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommandList {
    list: Vec<UnitKind>,
    ignore_history: bool,
    current: usize,
}

impl CommandList {
    pub fn new(list: Vec<UnitKind>, ignore_history: bool) -> Self {
        Self {
            list,
            ignore_history,
            current: 0,
        }
    }
}

impl Iterator for CommandList {
    type Item = UnitKind;

    fn next(&mut self) -> Option<UnitKind> {
        if self.current >= self.list.len() {
            None
        } else {
            let result = self.list[self.current].clone();
            self.current += 1;
            Some(result)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnitKind {
    SimpleCommand {
        command: Vec<WordList>,
        redirect: Vec<Redirect>,
        background: bool,
    },
    Connecter {
        left: Box<UnitKind>,
        right: Box<UnitKind>,
        kind: ConnecterKind,
        background: bool,
    },
    If {
        condition: Box<UnitKind>,
        true_case: Vec<UnitKind>,
        false_case: Option<Vec<UnitKind>>,
        redirect: Vec<Redirect>,
        background: bool,
    },
    Unless {
        condition: Box<UnitKind>,
        false_case: Vec<UnitKind>,
        true_case: Option<Vec<UnitKind>>,
        redirect: Vec<Redirect>,
        background: bool,
    },
    While {
        condition: Box<UnitKind>,
        command: Vec<UnitKind>,
        redirect: Vec<Redirect>,
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

#[macro_export]
macro_rules! loc {
    ($c: expr, $l: expr) => {
        Location::new($c, $l)
    };
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

    pub fn location(&self) -> Location {
        self.loc
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

    pub fn invalid_fd(s: &str, loc: Location) -> Self {
        Self::new(ParseErrorKind::InvalidFd(s.to_string()), loc)
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

pub fn parse_command_line(s: &str) -> Result<CommandList, ParseError> {
    let tokens = lex(s)?;
    let mut tokens = TokenReader::new(tokens);
    let mut result = vec![];

    // If it starts with a Space, ignore the command history.
    let ignore_history = matches!(tokens.skip_space(), Some(_));

    loop {
        match parse_command(&mut tokens)? {
            None => break,
            Some(c) => result.push(c),
        }
    }

    let result = CommandList::new(result, ignore_history);
    Ok(result)
}