use super::word::WordKind;
use crate::location::{Annotate, Location};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Space,
    Word(String, WordKind),
    Number(String),
    Comment(String),
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
    WriteCopy,    // 'm>&n'
    Append,       // '>>'
    AppendBoth,   // '&>>'
    ReadClose,    // '<&-'
    WriteClose,   // '>&-'
    ReadWrite,    // '<>'
    HereDocument, // '<<'
    HereString,   // '<<<'
    Termination,  // ';'
    GroupStart,   // '{'
    GroupEnd,     // '}'
    Hyphen,
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
    Until,
    For,
    In,
}
pub type Token = Annotate<TokenKind>;

impl Token {
    pub fn space(loc: Location) -> Self {
        Self::new(TokenKind::Space, loc)
    }

    pub fn word<S: AsRef<str>>(s: S, k: WordKind, loc: Location) -> Self {
        Self::new(TokenKind::Word(String::from(s.as_ref()), k), loc)
    }

    pub fn number(n: String, loc: Location) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }

    pub fn comment(c: String, loc: Location) -> Self {
        Self::new(TokenKind::Comment(c), loc)
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

    pub fn here_document(loc: Location) -> Self {
        Self::new(TokenKind::HereDocument, loc)
    }

    pub fn here_string(loc: Location) -> Self {
        Self::new(TokenKind::HereDocument, loc)
    }

    pub fn termination(loc: Location) -> Self {
        Self::new(TokenKind::Termination, loc)
    }

    pub fn group_start(loc: Location) -> Self {
        Self::new(TokenKind::GroupStart, loc)
    }

    pub fn group_end(loc: Location) -> Self {
        Self::new(TokenKind::GroupEnd, loc)
    }

    pub fn hyphen(loc: Location) -> Self {
        Self::new(TokenKind::Hyphen, loc)
    }

    pub fn newline(loc: Location) -> Self {
        Self::new(TokenKind::NewLine, loc)
    }

    pub fn keyword(k: &str, loc: Location) -> Self {
        let kind = match k {
            "if" => TokenKind::If,
            "then" => TokenKind::Then,
            "else" => TokenKind::Else,
            "elsif" => TokenKind::ElsIf,
            "elif" => TokenKind::ElIf,
            "fi" => TokenKind::Fi,
            "end" => TokenKind::End,
            "unless" => TokenKind::Unless,
            "while" => TokenKind::While,
            "do" => TokenKind::Do,
            "done" => TokenKind::Done,
            "until" => TokenKind::Until,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            _ => unimplemented![],
        };
        Self::new(kind, loc)
    }
}
