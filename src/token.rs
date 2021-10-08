use crate::parser::{Annotate, Location, RedirectKind, WordKind};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Space,
    Word(String, WordKind),
    Redirect(RedirectKind),
    Number(String),
    And,
    Pipe,
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

    pub fn redirect(r: RedirectKind, loc: Location) -> Self {
        Self::new(TokenKind::Redirect(r), loc)
    }

    pub fn number(n: String, loc: Location) -> Self {
        Self::new(TokenKind::Number(n), loc)
    }

    pub fn and(loc: Location) -> Self {
        Self::new(TokenKind::And, loc)
    }

    pub fn pipe(loc: Location) -> Self {
        Self::new(TokenKind::Pipe, loc)
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

    pub fn eof(loc: Location) -> Self {
        Self::new(TokenKind::Eof, loc)
    }
}
