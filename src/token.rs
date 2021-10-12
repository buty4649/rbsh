use crate::loc;
use crate::parser::word::WordKind;
use crate::parser::{Annotate, Location, ParseError};
use std::iter::Iterator;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Space,
    Word(String, WordKind),
    Number(String),
    Background,   // '&'
    Pipe,         // '|'
    PipeBoth,     // '|&'
    And,          // '&&'
    Or,           // '||'
    ReadFrom,     // '<'
    WriteTo,      // '>'
    ForceWriteTo, // '>|'
    WriteBoth,    // '&>', '>&'
    ReadCopy,     // '<&'
    WriteCopy,    // '>&'
    Append,       // '>>'
    AppendBoth,   // '&>>'
    ReadClose,    // '<&-'
    WriteClose,   // '>&-'
    ReadWrite,    // '<>'
    HereDocument, // '<<'
    HereString,   // '<<<'
    Hyphen,
    Termination, // ';'
    NewLine,
    If,
    Then,
    Else,
    ElIf,
    ElsIf,
    Fi,
    End,
    Unless,
    While,
    Do,
    Done,
    Eof,
}
pub type Token = Annotate<TokenKind>;

impl Token {
    pub fn space(loc: Location) -> Self {
        Self::new(TokenKind::Space, loc)
    }

    pub fn word(s: String, k: WordKind, loc: Location) -> Self {
        Self::new(TokenKind::Word(s, k), loc)
    }

    pub fn number(n: String, loc: Location) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }

    pub fn background(loc: Location) -> Self {
        Self::new(TokenKind::Background, loc)
    }

    pub fn pipe(loc: Location) -> Self {
        Self::new(TokenKind::Pipe, loc)
    }

    pub fn pipe_both(loc: Location) -> Self {
        Self::new(TokenKind::PipeBoth, loc)
    }

    pub fn and(loc: Location) -> Self {
        Self::new(TokenKind::And, loc)
    }

    pub fn or(loc: Location) -> Self {
        Self::new(TokenKind::Or, loc)
    }

    pub fn read_from(loc: Location) -> Self {
        Self::new(TokenKind::ReadFrom, loc)
    }

    pub fn write_to(loc: Location) -> Self {
        Self::new(TokenKind::WriteTo, loc)
    }

    pub fn force_write_to(loc: Location) -> Self {
        Self::new(TokenKind::ForceWriteTo, loc)
    }

    pub fn write_both(loc: Location) -> Self {
        Self::new(TokenKind::WriteBoth, loc)
    }

    pub fn read_copy(loc: Location) -> Self {
        Self::new(TokenKind::ReadCopy, loc)
    }

    pub fn write_copy(loc: Location) -> Self {
        Self::new(TokenKind::WriteCopy, loc)
    }

    pub fn append(loc: Location) -> Self {
        Self::new(TokenKind::Append, loc)
    }

    pub fn append_both(loc: Location) -> Self {
        Self::new(TokenKind::AppendBoth, loc)
    }

    pub fn read_close(loc: Location) -> Self {
        Self::new(TokenKind::ReadClose, loc)
    }

    pub fn write_close(loc: Location) -> Self {
        Self::new(TokenKind::WriteClose, loc)
    }

    pub fn read_write(loc: Location) -> Self {
        Self::new(TokenKind::ReadWrite, loc)
    }

    pub fn hyphen(loc: Location) -> Self {
        Self::new(TokenKind::Hyphen, loc)
    }

    pub fn here_document(loc: Location) -> Self {
        Self::new(TokenKind::HereDocument, loc)
    }

    pub fn here_string(loc: Location) -> Self {
        Self::new(TokenKind::HereDocument, loc)
    }

    pub fn termination(loc: Location) -> Self {
        Self::new(TokenKind::Termination, loc)
    }

    pub fn newline(loc: Location) -> Self {
        Self::new(TokenKind::NewLine, loc)
    }

    pub fn if_keyword(loc: Location) -> Self {
        Self::new(TokenKind::If, loc)
    }

    pub fn then_keyword(loc: Location) -> Self {
        Self::new(TokenKind::Then, loc)
    }

    pub fn else_keyword(loc: Location) -> Self {
        Self::new(TokenKind::Else, loc)
    }

    pub fn elsif_keyword(loc: Location) -> Self {
        Self::new(TokenKind::ElsIf, loc)
    }

    pub fn elif_keyword(loc: Location) -> Self {
        Self::new(TokenKind::ElIf, loc)
    }

    pub fn fi_keyword(loc: Location) -> Self {
        Self::new(TokenKind::Fi, loc)
    }

    pub fn end_keyword(loc: Location) -> Self {
        Self::new(TokenKind::End, loc)
    }

    pub fn unless_keyword(loc: Location) -> Self {
        Self::new(TokenKind::Unless, loc)
    }

    pub fn while_keyword(loc: Location) -> Self {
        Self::new(TokenKind::While, loc)
    }

    pub fn do_keyword(loc: Location) -> Self {
        Self::new(TokenKind::Do, loc)
    }

    pub fn done_keyword(loc: Location) -> Self {
        Self::new(TokenKind::Done, loc)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenReader {
    tokens: Vec<Token>,
    pos: usize,
}

impl TokenReader {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn current(&self) -> Option<Token> {
        if self.is_eof() || self.pos == 0 {
            None
        } else {
            Some(self.tokens[self.pos - 1].clone())
        }
    }

    pub fn peek(&mut self) -> Option<Token> {
        if self.is_eof() {
            None
        } else {
            let result = self.tokens[self.pos].clone();
            Some(result)
        }
    }

    pub fn peek_token(&mut self) -> Option<TokenKind> {
        match self.peek() {
            Some(t) => Some(t.value),
            None => None,
        }
    }

    pub fn to_vec(&self) -> Vec<Token> {
        self.tokens[self.pos..].to_vec()
    }

    pub fn skip_space(&mut self) -> Option<Token> {
        let mut last_token: Option<Token> = None;
        loop {
            match self.peek_token() {
                Some(TokenKind::Space) => {
                    last_token = self.next();
                }
                _ => break last_token,
            }
        }
    }

    pub fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    pub fn location(&self) -> Location {
        if self.tokens.is_empty() {
            loc!(0, 0)
        } else if self.is_eof() {
            let loc = self.tokens.last().unwrap().location();
            Location::new_from_offset(&loc, 1, 0)
        } else {
            self.tokens[self.pos].location()
        }
    }

    pub fn error_unexpected_token(&self) -> ParseError {
        if self.is_eof() {
            self.error_eof()
        } else {
            let token = self.tokens[self.pos].clone();
            ParseError::unexpected_token(token)
        }
    }

    pub fn error_invalid_fd(&self, fd: &str) -> ParseError {
        if self.is_eof() {
            self.error_eof()
        } else {
            ParseError::invalid_fd(fd, self.location())
        }
    }

    pub fn error_eof(&self) -> ParseError {
        ParseError::eof(self.location())
    }
}

impl Iterator for TokenReader {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match self.peek() {
            Some(v) => {
                self.pos += 1;
                Some(v)
            }
            None => None,
        }
    }
}

#[macro_export]
macro_rules! normal_word {
    ($s: expr, $l: expr) => {
        Token::word($s.to_string(), WordKind::Normal, $l)
    };
    ($s: expr) => {
        normal_word!($s, loc!(1, 1))
    };
}

#[macro_export]
macro_rules! quote_word {
    ($s: expr, $l: expr) => {
        Token::word($s.to_string(), WordKind::Quote, $l)
    };
    ($s: expr) => {
        quote_word!($s, loc!(1, 1))
    };
}

#[macro_export]
macro_rules! literal_word {
    ($s: expr, $l: expr) => {
        Token::word($s.to_string(), WordKind::Literal, $l)
    };
    ($s: expr) => {
        literal_word!($s, loc!(1, 1))
    };
}

#[macro_export]
macro_rules! cmd {
    ($s: expr, $l: expr) => {
        Token::word($s.to_string(), WordKind::Command, $l)
    };
    ($s: expr) => {
        cmd!($s, loc!(1, 1))
    };
}

#[macro_export]
macro_rules! var {
    ($s: expr, $l: expr) => {
        Token::word($s.to_string(), WordKind::Variable, $l)
    };
    ($s: expr) => {
        var!($s, loc!(1, 1))
    };
}

#[macro_export]
macro_rules! param {
    ($s: expr, $l: expr) => {
        Token::word($s.to_string(), WordKind::Parameter, $l)
    };
    ($s: expr) => {
        param!($s, loc!(1, 1))
    };
}

#[macro_export]
macro_rules! number {
    ($s: expr, $l: expr) => {
        Token::number($s.to_string(), $l)
    };
    ($s: expr) => {
        number!($s, loc!(1, 1))
    };
}
