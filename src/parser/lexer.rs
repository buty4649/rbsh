use super::{
    token::{Token, TokenKind},
    word::WordKind,
};
use crate::{error::ShellError, location::Location, status::Result};
use std::str::{from_utf8, Utf8Error};
type LexResult = Result<Token>;

#[derive(Debug, Clone)]
struct Lexer {
    input: Vec<u8>,
    pos: usize,
    line: usize,
    column: usize,
    token: Vec<Token>,
    begin_command: bool,
}

macro_rules! lex_simple_token {
    ($self: expr, $kind: path, $e: expr) => {{
        let loc = $self.location();
        while matches!($self.peek(), Some(c) if $e(c)) {
            $self.next();
        }
        Ok($kind(loc))
    }};
}

impl Lexer {
    fn new(input: &str) -> Self {
        let input = input.as_bytes().to_vec();
        Lexer {
            input,
            pos: 0,
            line: 1,
            column: 1,
            token: vec![],
            begin_command: true,
        }
    }

    fn lex(&mut self) -> Result<Vec<Token>> {
        if self.is_eof() {
            return Ok(vec![]);
        }

        macro_rules! action {
            ($f: ident) => {{
                let token = self.$f()?;
                self.push(token);
            }};
        }

        macro_rules! keyword {
            ($s: expr, $ident: ident) => {{
                let cnt = $s.len();
                let loc = self.location();
                (0..cnt).for_each(|_| {
                    self.next();
                });
                let token = Token::$ident(loc);
                self.push(token)
            }};
        }

        loop {
            match self.peek() {
                None => break,
                Some(b'&') => action!(lex_ampersand),
                Some(b'|') => action!(lex_vertical_line),
                Some(b'-') => action!(lex_hyphen),
                Some(b'$') => action!(lex_dollar),
                Some(b'`') => action!(lex_backquote),
                Some(b';') => action!(lex_semicolon),
                Some(b'\'') => action!(lex_single_quote),
                Some(b'\n') => action!(lex_newline),
                Some(b'<') | Some(b'>') => action!(lex_redirect),
                Some(c) if is_space(c) => action!(lex_space),
                Some(c) if is_number(c) => action!(lex_number),
                Some(b'"') => {
                    let mut tokens = self.lex_double_quote()?;
                    self.token.append(&mut tokens)
                }
                _ if self.starts_with("{") => keyword!("{", group_start),
                _ if self.starts_with("}") => keyword!("}", group_end),
                _ if self.starts_with("if") => keyword!("if", if_keyword),
                _ if self.starts_with("then") => keyword!("then", then_keyword),
                _ if self.starts_with("fi") => keyword!("fi", fi_keyword),
                _ if self.starts_with("else") => keyword!("else", else_keyword),
                _ if self.starts_with("elsif") => keyword!("elsif", elsif_keyword),
                _ if self.starts_with("elif") => keyword!("elif", elif_keyword),
                _ if self.starts_with("end") => keyword!("end", end_keyword),
                _ if self.starts_with("unless") => keyword!("unless", unless_keyword),
                _ if self.starts_with("while") => keyword!("while", while_keyword),
                _ if self.starts_with("do") => keyword!("do", do_keyword),
                _ if self.starts_with("done") => keyword!("done", done_keyword),
                _ if self.starts_with("until") => keyword!("until", until_keyword),
                _ if self.starts_with("for") => keyword!("for", for_keyword),
                _ if self.is_in_keyword() => keyword!("in", in_keyword),
                _ => action!(lex_word),
            }

            self.begin_command = match self.before_token() {
                Some(TokenKind::Termination)
                | Some(TokenKind::NewLine)
                | Some(TokenKind::GroupStart)
                | Some(TokenKind::If)
                | Some(TokenKind::Then)
                | Some(TokenKind::Else)
                | Some(TokenKind::ElIf)
                | Some(TokenKind::ElsIf)
                | Some(TokenKind::Pipe)
                | Some(TokenKind::PipeBoth)
                | Some(TokenKind::And)
                | Some(TokenKind::Or)
                | Some(TokenKind::Unless)
                | Some(TokenKind::While)
                | Some(TokenKind::Do)
                | Some(TokenKind::Until) => true,
                Some(TokenKind::Space) if self.begin_command => true,
                _ => false,
            }
        }

        Ok(self.token.to_vec())
    }

    fn is_in_keyword(&mut self) -> bool {
        // "For" "Space" "Word" "Space" "In"
        let len = self.token.len();
        len >= 4 && self.token[len - 4].value() == TokenKind::For && {
            let tmp = self.begin_command;
            self.begin_command = true;
            let result = self.starts_with("in");
            self.begin_command = tmp;
            result
        }
    }

    fn lex_space(&mut self) -> LexResult {
        lex_simple_token!(self, Token::space, is_space)
    }

    fn lex_newline(&mut self) -> LexResult {
        lex_simple_token!(self, Token::newline, |c| c == b'\n')
    }

    fn lex_semicolon(&mut self) -> LexResult {
        lex_simple_token!(self, Token::termination, |c| c == b';')
    }

    fn lex_ampersand(&mut self) -> LexResult {
        let loc = self.location();
        self.next(); // '&'
        let token = match self.peek() {
            Some(b'&') => {
                self.next();
                Token::and(loc)
            }
            Some(b'>') => {
                self.next();
                match self.peek() {
                    Some(b'>') => {
                        self.next();
                        Token::append_both(loc)
                    }
                    _ => Token::write_both(loc),
                }
            }
            _ => Token::background(loc),
        };
        Ok(token)
    }

    fn lex_vertical_line(&mut self) -> LexResult {
        let loc = self.location();
        self.next(); // '|'
        let token = match self.peek() {
            Some(b'&') => {
                self.next();
                Token::pipe_both(loc)
            }
            Some(b'|') => {
                self.next();
                Token::or(loc)
            }
            _ => Token::pipe(loc),
        };
        Ok(token)
    }

    fn lex_hyphen(&mut self) -> LexResult {
        let loc = self.location();
        self.next(); // '-'
        let token = if matches!(self.before_token(), Some(TokenKind::Number(_))) {
            Token::hyphen(loc)
        } else {
            Token::word("-".to_string(), WordKind::Normal, loc)
        };
        Ok(token)
    }

    fn lex_number(&mut self) -> LexResult {
        let loc = self.location();
        let mut result = String::new();
        while matches!(self.peek(), Some(c) if is_number(c)) {
            result.push(self.next().unwrap() as char);
        }

        let check = matches!(self.peek(), Some(b'<') | Some(b'>'))
            || matches!(
                self.before_token(),
                Some(TokenKind::ReadCopy) | Some(TokenKind::WriteCopy)
            );
        let token = if check {
            Token::number(result, loc)
        } else {
            Token::word(result, WordKind::Normal, loc)
        };
        Ok(token)
    }

    fn lex_redirect(&mut self) -> LexResult {
        match self.peek() {
            Some(b'<') => self.lex_redirect_less(),
            Some(b'>') => self.lex_redirect_grater(),
            _ => unreachable![],
        }
    }

    fn lex_redirect_less(&mut self) -> LexResult {
        let loc = self.location();
        self.next(); // '<'
        let token = match self.next_if(|c| matches!(c, b'<' | b'>' | b'&')) {
            Some(b'<') => match self.next_if(|c| c == b'<') {
                // '<<<'
                Some(b'<') => Token::here_string(loc),
                // '<<'
                _ => Token::here_document(loc),
            },
            // '<>'
            Some(b'>') => Token::read_write(loc),
            Some(b'&') => match self.next_if(|c| c == b'-') {
                // '<&-'
                Some(b'-') => Token::read_close(loc),
                // '<&'
                _ => Token::read_copy(loc),
            },
            // '<'
            _ => Token::read_from(loc),
        };
        Ok(token)
    }

    fn lex_redirect_grater(&mut self) -> LexResult {
        let loc = self.location();
        self.next(); // '>'
        let token = match self.next_if(|c| matches!(c, b'>' | b'|' | b'&')) {
            // '>>loc'
            Some(b'>') => Token::append(loc),
            // '>|'
            Some(b'|') => Token::force_write_to(loc),
            Some(b'&') => match self.next_if(|c| c == b'-') {
                // '>&-'
                Some(b'-') => Token::write_close(loc),
                // '>&'
                _ => {
                    let is_write_copy = matches!(self.peek(), Some(c) if is_number(c))
                        || matches!(self.before_token(), Some(TokenKind::Number(_)));
                    if is_write_copy {
                        Token::write_copy(loc)
                    } else {
                        Token::write_both(loc)
                    }
                }
            },
            // '>'
            _ => Token::write_to(loc),
        }; //
        Ok(token)
    }

    fn lex_dollar(&mut self) -> LexResult {
        let token = match self.peek_nth(2) {
            Some(b'{') => self.lex_parameter()?,
            Some(b'(') => self.lex_command_substitute()?,
            Some(_) => self.lex_variable()?,
            None => {
                let loc = self.location();
                self.next();
                Token::word('$'.to_string(), WordKind::Normal, loc)
            }
        };
        Ok(token)
    }

    fn lex_parameter(&mut self) -> LexResult {
        let loc = self.location();

        self.next(); // '$'
        self.next(); // '{'
        let word = self.lex_internal_word(false, |c| c == b'}')?;
        self.next(); // '}'

        let token = Token::word(word, WordKind::Parameter, loc);
        Ok(token)
    }

    fn lex_backquote(&mut self) -> LexResult {
        let loc = self.location();
        self.next(); // '`'
        let word = self.lex_internal_word(false, |c| c == b'`')?;
        self.next(); // '`'
        let token = Token::word(word, WordKind::Command, loc);
        Ok(token)
    }

    fn lex_single_quote(&mut self) -> LexResult {
        let loc = self.location();
        self.next(); // '\''
        let word = self.lex_internal_word(false, |c| c == b'\'')?;
        self.next(); // '\''
        let token = Token::word(word, WordKind::Literal, loc);
        Ok(token)
    }

    fn lex_internal_word(
        &mut self,
        allow_eof: bool,
        terminator: impl Fn(u8) -> bool,
    ) -> Result<String> {
        let mut result = vec![];
        loop {
            match self.peek() {
                Some(b'\\') => {
                    self.next();
                    match self.peek() {
                        Some(c) if terminator(c) => {
                            self.next();
                            result.push(c);
                        }
                        _ => result.push(b'\\'),
                    }
                }
                Some(c) if terminator(c) => break,
                Some(c) => {
                    self.next();
                    result.push(c);
                }
                None if allow_eof => break,
                None => return Err(ShellError::eof(self.location())),
            }
        }
        let result = from_utf8(&*result).map_err(|e| self.error_invalid_utf8_sequence(e))?;
        Ok(result.to_string())
    }

    fn lex_double_quote(&mut self) -> Result<Vec<Token>> {
        let mut loc = self.location();
        self.next(); // '"'
        let mut result = vec![];

        let terminator = |c| matches!(c, b'"' | b'`' | b'$');
        loop {
            let word = self.lex_internal_word(false, terminator)?;
            let token = Token::word(word, WordKind::Quote, loc);
            result.push(token);

            let token = match self.peek() {
                Some(b'"') => break,
                Some(b'`') => self.lex_backquote()?,
                Some(b'$') => self.lex_dollar()?,
                _ => unreachable![],
            };
            result.push(token);

            loc = self.location();
        }
        self.next(); // '"'

        Ok(result)
    }

    fn lex_word(&mut self) -> LexResult {
        let loc = self.location();
        let terminator =
            |c| is_space(c) || is_line_ending(c) || is_quote(c) || is_symbol(c) || c == b'$';

        let word = self.lex_internal_word(true, terminator)?;
        let token = Token::word(word, WordKind::Normal, loc);
        Ok(token)
    }

    fn lex_command_substitute(&mut self) -> LexResult {
        let mut result = vec![];
        let mut nest = 0;
        let loc = self.location();

        self.next(); // '$'
        self.next(); // '('

        loop {
            match self.peek() {
                Some(b')') if nest == 0 => break,
                Some(b')') => {
                    result.push(self.next().unwrap());
                    nest -= 1;
                }
                Some(b'$') => {
                    result.push(self.next().unwrap());
                    // "$("
                    if matches!(self.peek(), Some(b'(')) {
                        nest += 1;
                        result.push(self.next().unwrap());
                    }
                }
                Some(c) => {
                    self.next();
                    result.push(c)
                }
                None => return Err(self.error_eof()),
            }
        }
        self.next(); // ')'

        let result = from_utf8(&*result).map_err(|e| self.error_invalid_utf8_sequence(e))?;
        Ok(Token::word(result.to_string(), WordKind::Command, loc))
    }

    fn lex_variable(&mut self) -> LexResult {
        let terminator = |c| {
            is_space(c)
                || is_line_ending(c)
                || is_quote(c)
                || is_symbol(c)
                || c == b'-'
                || c == b':'
        };
        let loc = self.location();
        self.next(); // '$'

        let word = match self.peek() {
            Some(b'$') => {
                self.next();
                "$".to_string()
            }
            _ => self.lex_internal_word(true, terminator)?,
        };
        let token = Token::word(word, WordKind::Variable, loc);
        Ok(token)
    }

    fn next(&mut self) -> Option<u8> {
        if self.is_eof() {
            None
        } else {
            let result = self.input[self.pos];
            if result == b'\n' {
                self.line += 1;
                self.column = 0;
            }
            self.pos += 1;
            self.column += 1;
            Some(result)
        }
    }

    fn next_if<F>(&mut self, f: F) -> Option<u8>
    where
        F: Fn(u8) -> bool,
    {
        match self.peek() {
            Some(c) if f(c) => self.next(),
            _ => None,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.peek_nth(1)
    }

    fn peek_nth(&self, offset: usize) -> Option<u8> {
        let offset = offset - 1;
        if self.pos + offset >= self.input.len() {
            None
        } else {
            Some(self.input[self.pos + offset])
        }
    }

    fn starts_with(&self, s: &str) -> bool {
        let len = s.len();
        self.begin_command
            && self.input[self.pos..].starts_with(s.as_bytes())
            && (self.pos + len >= self.input.len()
                || (self.pos + len < self.input.len() && {
                    let c = self.input[self.pos + len];
                    is_space(c) || is_line_ending(c) || c == b';' || c == b'&'
                }))
    }

    fn before_token(&self) -> Option<TokenKind> {
        self.token.last().map(|t| t.value())
    }

    fn push(&mut self, t: Token) {
        self.token.push(t);
    }

    fn location(&self) -> Location {
        Location::new(self.column, self.line)
    }

    fn is_eof(&self) -> bool {
        self.input.is_empty() || self.pos >= self.input.len()
    }

    fn error_invalid_utf8_sequence(&self, err: Utf8Error) -> ShellError {
        ShellError::invalid_utf8_sequence(err, self.location())
    }

    fn error_eof(&self) -> ShellError {
        ShellError::eof(self.location())
    }
}

pub fn lex(input: &str) -> Result<Vec<Token>> {
    Lexer::new(input).lex()
}

fn is_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t')
}

fn is_line_ending(c: u8) -> bool {
    c == b'\n'
}

fn is_quote(c: u8) -> bool {
    matches!(c, b'"' | b'\\' | b'`')
}

fn is_symbol(c: u8) -> bool {
    matches!(c, b'$' | b';' | b'&' | b'|' | b'<' | b'>')
}

fn is_number(c: u8) -> bool {
    matches!(c, b'0'..=b'9')
}

include!("lexer_test.rs");
