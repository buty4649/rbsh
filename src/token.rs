use crate::parser::word::WordKind;
use crate::parser::{Annotate, Location};

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

    pub fn eof(loc: Location) -> Self {
        Self::new(TokenKind::Eof, loc)
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
