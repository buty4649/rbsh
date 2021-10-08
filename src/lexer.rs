use crate::parser::{Location, ParseError, WordKind};
use crate::token::{Token, TokenKind};
use std::str::{from_utf8, Utf8Error};
type LexResult = Result<Token, ParseError>;

#[derive(Debug, Clone)]
struct Lexer {
    input: Vec<u8>,
    pos: usize,
    line: usize,
    column: usize,
    token: Vec<Token>,
}

impl Lexer {
    fn new(input: &str) -> Self {
        let mut v = input.as_bytes().to_vec();
        let mut input = vec![0]; // dummy
        input.append(&mut v);
        Lexer {
            input,
            pos: 0,
            line: 1,
            column: 1,
            token: vec![],
        }
    }

    fn lex(&mut self) -> Result<Vec<Token>, ParseError> {
        if self.is_eof() {
            return Err(self.error_eof());
        }

        macro_rules! action {
            ($f: ident) => {{
                let token = self.$f()?;
                self.push(token);
            }};
        }

        loop {
            match self.peek() {
                Some(b'&') => action!(lex_ampersand),
                Some(b'|') => action!(lex_vertical_line),
                Some(b'-') => action!(lex_hyphen),
                Some(b'$') => action!(lex_dollar),
                Some(b'`') => action!(lex_backquote),
                Some(b'\'') => action!(lex_single_quote),
                Some(b'<') | Some(b'>') => action!(lex_redirect),
                Some(c) if is_space(c) => action!(lex_space),
                Some(c) if is_number(c) => action!(lex_number),
                None => break,
                Some(b'"') => {
                    let mut tokens = self.lex_double_quote()?;
                    self.token.append(&mut tokens)
                }
                _ => action!(lex_word),
            }
        }

        Ok(self.token.to_vec())
    }

    fn lex_space(&mut self) -> LexResult {
        let loc = self.location();
        while matches!(self.peek(), Some(c) if is_space(c)) {
            self.next();
        }
        Ok(Token::space(loc))
    }

    fn lex_ampersand(&mut self) -> LexResult {
        let loc = self.location();
        self.next(); // '&'
        let token = match self.peek() {
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
            _ => Token::and(loc),
        };
        Ok(token)
    }

    fn lex_vertical_line(&mut self) -> LexResult {
        let loc = self.location();
        self.next(); // '|'
        Ok(Token::pipe(loc))
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
                    let is_write_both = matches!(self.peek(), Some(c) if is_number(c))
                        || !matches!(self.before_token(), Some(TokenKind::Number(_)));
                    if is_write_both {
                        Token::write_both(loc)
                    } else {
                        Token::write_copy(loc)
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
    ) -> Result<String, ParseError> {
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
                None => return Err(ParseError::eof(self.location())),
            }
        }
        let result = from_utf8(&*result).map_err(|e| self.error_invalid_utf8_sequence(e))?;
        Ok(result.to_string())
    }

    fn lex_double_quote(&mut self) -> Result<Vec<Token>, ParseError> {
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

        let word = self.lex_internal_word(true, terminator)?;
        let token = Token::word(word, WordKind::Variable, loc);
        Ok(token)
    }

    fn next(&mut self) -> Option<u8> {
        self.pos += 1;
        if self.is_eof() {
            None
        } else {
            let result = self.input[self.pos];
            if result == b'\n' {
                self.line += 1;
                self.column = 0;
            }
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
        if self.pos + offset >= self.input.len() {
            None
        } else {
            Some(self.input[self.pos + offset])
        }
    }

    fn before_token(&self) -> Option<TokenKind> {
        self.token.last().map(|t| t.value.clone())
    }

    fn push(&mut self, t: Token) {
        self.token.push(t);
    }

    fn location(&self) -> Location {
        Location::new(self.column, self.line)
    }

    fn is_eof(&self) -> bool {
        self.input.len() == 1 || self.pos >= self.input.len()
    }

    fn error_invalid_utf8_sequence(&self, err: Utf8Error) -> ParseError {
        ParseError::invalid_utf8_sequence(err, self.location())
    }

    fn error_eof(&self) -> ParseError {
        ParseError::eof(self.location())
    }
}

pub fn lex(input: &str) -> Result<Vec<Token>, ParseError> {
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
