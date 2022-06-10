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
        Self::new(TokenKind::HereString, loc)
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::location;

    macro_rules! assert_token {
        ($left:ident, $right:path) => {{
            let token = Token::$left(location!());
            assert_eq!(token.value, $right);
            assert_eq!(token.location, location!());
        }};

        ($left:ident, $a:expr, $right:path) => {{
            let token = Token::$left($a, location!());
            assert_eq!(token.value, $right($a));
            assert_eq!(token.location, location!());
        }};

        ($left:ident, $a1:expr, $a2:expr, $right:path) => {{
            let token = Token::$left($a1, $a2, location!());
            assert_eq!(token.value, $right($a1.to_string(), $a2));
            assert_eq!(token.location, location!());
        }};
    }

    macro_rules! assert_keyword {
        ($left:expr, $right:path) => {{
            let token = Token::keyword($left, location!());
            assert_eq!(token.value, $right);
            assert_eq!(token.location, location!());
        }};
    }

    #[test]
    fn token() {
        assert_token!(space, TokenKind::Space);
        assert_token!(word, "abc", WordKind::Normal, TokenKind::Word);
        assert_token!(number, "1".to_string(), TokenKind::Number);
        assert_token!(comment, "test".to_string(), TokenKind::Comment);
        assert_token!(background, TokenKind::Background);
        assert_token!(pipe, TokenKind::Pipe);
        assert_token!(pipe_both, TokenKind::PipeBoth);
        assert_token!(and, TokenKind::And);
        assert_token!(or, TokenKind::Or);
        assert_token!(read_from, TokenKind::ReadFrom);
        assert_token!(write_to, TokenKind::WriteTo);
        assert_token!(force_write_to, TokenKind::ForceWriteTo);
        assert_token!(write_both, TokenKind::WriteBoth);
        assert_token!(read_copy, TokenKind::ReadCopy);
        assert_token!(write_copy, TokenKind::WriteCopy);
        assert_token!(append, TokenKind::Append);
        assert_token!(append_both, TokenKind::AppendBoth);
        assert_token!(read_close, TokenKind::ReadClose);
        assert_token!(write_close, TokenKind::WriteClose);
        assert_token!(read_write, TokenKind::ReadWrite);
        assert_token!(here_document, TokenKind::HereDocument);
        assert_token!(here_string, TokenKind::HereString);
        assert_token!(termination, TokenKind::Termination);
        assert_token!(group_start, TokenKind::GroupStart);
        assert_token!(group_end, TokenKind::GroupEnd);
        assert_token!(hyphen, TokenKind::Hyphen);
        assert_token!(newline, TokenKind::NewLine);

        assert_keyword!("if", TokenKind::If);
        assert_keyword!("then", TokenKind::Then);
        assert_keyword!("else", TokenKind::Else);
        assert_keyword!("elsif", TokenKind::ElsIf);
        assert_keyword!("elif", TokenKind::ElIf);
        assert_keyword!("fi", TokenKind::Fi);
        assert_keyword!("end", TokenKind::End);
        assert_keyword!("unless", TokenKind::Unless);
        assert_keyword!("while", TokenKind::While);
        assert_keyword!("do", TokenKind::Do);
        assert_keyword!("done", TokenKind::Done);
        assert_keyword!("until", TokenKind::Until);
        assert_keyword!("for", TokenKind::For);
        assert_keyword!("in", TokenKind::In);
    }
}
