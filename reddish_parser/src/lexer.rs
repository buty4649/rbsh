mod iterator;
mod reader;
mod utils;

pub use iterator::LexerIterator;

use crate::{error::Error, location::Location, Result, Token, TokenKind, WordKind};
use reader::Reader;
use utils::*;

#[derive(Debug)]
pub struct Lexer {
    reader: Reader,
    quoted_word_location: Option<Location>,
    before_token: Option<TokenKind>,
    head: bool,
    statement: Option<Statement>,
}

#[derive(Debug)]
enum Statement {
    For,
}

macro_rules! token {
    ($name:ident) => {
        fn $name(&mut self) -> Result<Token> {
            let location = self.reader.location();
            self.reader.next();
            Ok(Token::$name(location))
        }
    };

    ($name: ident, $f:expr) => {
        fn $name(&mut self) -> Result<Token> {
            let location = self.reader.location();
            while self.reader.next_if($f).is_some() {}
            Ok(Token::$name(location))
        }
    };
}

impl Lexer {
    pub fn new(input: &str, line: usize) -> Self {
        Lexer {
            reader: Reader::new(input, line),
            quoted_word_location: None,
            before_token: None,
            head: true,
            statement: None,
        }
    }

    pub fn location(&self) -> Location {
        self.reader.location()
    }

    pub fn lex(&mut self) -> Option<Result<Token>> {
        macro_rules! keyword {
            ($name:expr) => {{
                let location = self.reader.location();
                self.reader.skip($name.len());
                Ok(Token::keyword($name, location))
            }};
        }

        self.reader.peek().map(|c| {
            let token = match c {
                _ if is_space(c) => self.space(),
                _ if is_newline(c) => self.newline(),
                _ if is_number(c) => self.number(),
                _ if is_single_quote(c) => {
                    self.word(WordKind::Quote, is_single_quote, true, true, false)
                }

                _ if self.head && self.starts_with("if") => keyword!("if"),
                _ if self.head && self.starts_with("then") => keyword!("then"),
                _ if self.head && self.starts_with("else") => keyword!("else"),
                _ if self.head && self.starts_with("elsif") => keyword!("elsif"),
                _ if self.head && self.starts_with("elif") => keyword!("elif"),
                _ if self.head && self.starts_with("fi") => keyword!("fi"),
                _ if self.head && self.starts_with("end") => keyword!("end"),
                _ if self.head && self.starts_with("unless") => keyword!("unless"),
                _ if self.head && self.starts_with("while") => keyword!("while"),
                _ if self.head && self.starts_with("do") => keyword!("do"),
                _ if self.head && self.starts_with("done") => keyword!("done"),
                _ if self.head && self.starts_with("until") => keyword!("until"),
                _ if self.head && self.starts_with("for") => {
                    self.statement = Some(Statement::For);
                    keyword!("for")
                }
                _ if matches!(self.statement, Some(Statement::For)) && self.starts_with("in") => {
                    self.statement = None;
                    keyword!("in")
                }

                ';' => self.termination(),
                '&' => self.ampersand(),
                '#' => self.comment(),
                '|' => self.vertical_line(),
                '<' => self.less_than(),
                '>' => self.greater_than(),
                '{' => self.group_start(),
                '}' => self.group_end(),
                '-' if matches!(self.before_token, Some(TokenKind::Number { .. })) => self.hyphen(),

                _ if self.quoted_word_location.is_some() => self.quoted_word(),
                '"' => self.quoted_word(),
                '$' => self.dollar_word(),
                _ => self.normal_word(),
            };

            if token.is_ok() {
                let kind = token.clone().unwrap().value;
                self.head = match kind {
                    TokenKind::Space => self.head,
                    TokenKind::NewLine
                    | TokenKind::Termination
                    | TokenKind::And
                    | TokenKind::Comment { .. }
                    | TokenKind::Pipe
                    | TokenKind::GroupStart
                    | TokenKind::GroupEnd
                    | TokenKind::If
                    | TokenKind::Then
                    | TokenKind::Else
                    | TokenKind::ElsIf
                    | TokenKind::ElIf
                    | TokenKind::Fi
                    | TokenKind::End
                    | TokenKind::Unless
                    | TokenKind::While
                    | TokenKind::Until
                    | TokenKind::Do
                    | TokenKind::Done => true,
                    _ => false,
                };

                self.before_token = Some(kind);
            }
            token
        })
    }

    token!(space, is_space);
    token!(newline, is_newline);
    token!(termination, is_termination);
    token!(group_start);
    token!(group_end);
    token!(hyphen);

    fn number(&mut self) -> Result<Token> {
        let mut iter = self.reader.iter();
        if loop {
                match iter.next() {
                    Some(c) if is_number(*c) => continue,
                    Some('<') | Some('>') => break true,
                    _ => break false,
                }
            } ||
            // e.g. <&3, >&2
            matches!(
                self.before_token,
                Some(TokenKind::ReadCopy) | Some(TokenKind::WriteCopy)
            )
        {
            let location = self.reader.location();
            let mut result = String::new();
            while let Some(c) = self.reader.next_if(is_number) {
                result.push(c);
            }
            Ok(Token::number(result, location))
        } else {
            self.normal_word()
        }
    }

    fn ampersand(&mut self) -> Result<Token> {
        let location = self.reader.location();
        self.reader.next(); // remove '&'

        match self.reader.next_if(|c| c == '&' || c == '>') {
            Some('>') => match self.reader.next_if(|c| c == '>') {
                Some(_) => Ok(Token::append_both(location)), // &>>
                _ => Ok(Token::write_both(location)),        // &>
            },
            Some('&') => Ok(Token::and(location)), // &&
            _ => Ok(Token::background(location)),  // &
        }
    }

    fn comment(&mut self) -> Result<Token> {
        let location = self.reader.location();
        self.reader.next(); // remove '#'
        let mut result = String::new();

        while let Some(c) = self.reader.next_not_if(is_newline) {
            result.push(c)
        }

        Ok(Token::comment(result, location))
    }

    fn vertical_line(&mut self) -> Result<Token> {
        let location = self.reader.location();
        self.reader.next(); // remove '|'

        match self.reader.next_if(|c| c == '|' || c == '&') {
            Some('|') => Ok(Token::or(location)),
            Some('&') => Ok(Token::pipe_both(location)),
            _ => Ok(Token::pipe(location)),
        }
    }

    fn less_than(&mut self) -> Result<Token> {
        let location = self.reader.location();
        self.reader.next(); // remove '|'

        match self.reader.next_if(|c| c == '<' || c == '&' || c == '>') {
            Some('<') => match self.reader.next_if(|c| c == '<') {
                Some('<') => Ok(Token::here_string(location)), // <<<
                _ => Ok(Token::here_document(location)),       // <<
            },
            Some('&') => match self.reader.next_if(|c| c == '-') {
                Some('-') => Ok(Token::read_close(location)), // <&-
                _ => Ok(Token::read_copy(location)),          // <&
            },
            Some('>') => Ok(Token::read_write(location)), // <>
            _ => Ok(Token::read_from(location)),          // <
        }
    }

    fn greater_than(&mut self) -> Result<Token> {
        let location = self.reader.location();
        self.reader.next(); // remove '|'

        match self.reader.next_if(|c| c == '&' || c == '|' || c == '>') {
            Some('&') => match self.reader.next_if(|c| c == '-') {
                Some('-') => Ok(Token::write_close(location)), // >&-
                _ => {
                    if matches!(self.reader.peek(), Some(c) if is_number(c))
                        || matches!(self.before_token, Some(TokenKind::Number { .. }))
                    {
                        Ok(Token::write_copy(location)) // m>&n
                    } else {
                        Ok(Token::write_both(location)) // >&
                    }
                }
            },
            Some('|') => Ok(Token::force_write_to(location)), // >|
            Some('>') => Ok(Token::append(location)),         // >>
            _ => Ok(Token::write_to(location)),               // >
        }
    }

    fn quoted_word(&mut self) -> Result<Token> {
        macro_rules! unterminated_string {
            () => {{
                let location = self.quoted_word_location.unwrap();
                self.quoted_word_location = None;
                Err(Error::unterminated_string(location))
            }};
        }
        macro_rules! retrive_word {
            () => {{
                let f = |c| is_double_quote(c) || c == '$';
                self.word(WordKind::Normal, f, false, true, false)
            }};
        }
        let token = match self.reader.peek() {
            Some('$') => self.dollar_word(),
            Some(c) => {
                if c == '"' {
                    let location = self.reader.location();
                    self.quoted_word_location = Some(location);
                    self.reader.next(); // remove '"'
                }
                retrive_word![]
            }
            None => unterminated_string![],
        }?;

        if self.reader.is_eof() {
            unterminated_string![]
        } else {
            if self.reader.next_if(is_double_quote).is_some() {
                self.quoted_word_location = None;
            }
            Ok(token)
        }
    }

    fn dollar_word(&mut self) -> Result<Token> {
        match self.reader.peek_nth(1) {
            Some('{') => {
                self.reader.next(); // remove '$'
                self.word(WordKind::Parameter, |c| c == '}', true, true, false)
            }
            Some('(') => {
                self.reader.next(); // remove '$'
                self.word(WordKind::Parameter, |c| c == ')', true, true, false)
            }
            Some(c) if !c.is_ascii_punctuation() => {
                self.reader.next(); // remove '$'
                let f = |c| is_normal_word_delimiter(c) || c == '\\';
                self.word(WordKind::Variable, f, false, false, false)
            }
            _ => self.normal_word(),
        }
    }

    fn normal_word(&mut self) -> Result<Token> {
        self.word(
            WordKind::Normal,
            is_normal_word_delimiter,
            false,
            true,
            true,
        )
    }

    fn word(
        &mut self,
        kind: WordKind,
        f: impl Fn(char) -> bool,
        surround: bool,
        escape: bool,
        remove_backslash: bool,
    ) -> Result<Token> {
        let location = self.reader.location();
        let mut result = String::new();

        // remove first char
        if surround {
            self.reader.next();
        }

        while let Some(c) = self.reader.next_not_if(&f) {
            match c {
                '\\' if escape && matches!(self.reader.peek(), Some(c) if f(c)) => {
                    result.push(self.reader.next().unwrap())
                }
                '\\' if remove_backslash => (),
                _ => result.push(c),
            }
        }
        // check termination
        if surround && self.reader.next_if(&f).is_none() {
            Err(Error::unterminated_string(location))
        } else {
            Ok(Token::word(result, kind, location))
        }
    }

    fn starts_with(&self, s: &str) -> bool {
        self.reader.starts_with(s)
            && (matches!(self.reader.peek_nth(s.len()), Some(c) if is_space(c) || is_newline(c) || is_termination(c))
                || self.reader.peek_nth(s.len()).is_none())
    }
}
