use super::{Lexer, Location};
use crate::{Result, Token, TokenKind};
use std::iter::Iterator;

#[derive(Debug)]
pub struct LexerIterator {
    lexer: Lexer,
    peeked: Option<Option<Result<Token>>>,
}

impl Lexer {
    pub fn iter(self) -> LexerIterator {
        LexerIterator {
            lexer: self,
            peeked: None,
        }
    }
}

impl Iterator for LexerIterator {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.peeked.take() {
            Some(v) => v,
            None => self.lexer.lex(),
        }
    }
}

impl LexerIterator {
    pub fn peek(&mut self) -> Option<&Result<Token>> {
        let lexer = &mut self.lexer;
        self.peeked.get_or_insert_with(|| lexer.lex()).as_ref()
    }

    pub fn next_if<F>(&mut self, f: F) -> Option<Result<Token>>
    where
        F: FnOnce(&TokenKind) -> bool,
    {
        match self.next() {
            Some(Ok(token)) if f(&token.value) => Some(Ok(token)),
            Some(Err(e)) => Some(Err(e)),
            other => {
                self.peeked = Some(other);
                None
            }
        }
    }

    pub fn skip_if_space(&mut self) -> Result<bool> {
        match self.next_if(|token| token == &TokenKind::Space) {
            None => Ok(false),
            Some(Err(e)) => Err(e),
            Some(Ok(_)) => Ok(true),
        }
    }

    pub fn skip_if_space_or_newline(&mut self) -> Result<bool> {
        match self.next_if(|token| matches!(token, &TokenKind::Space | &TokenKind::NewLine)) {
            None => Ok(false),
            Some(Err(e)) => Err(e),
            Some(Ok(_)) => Ok(true),
        }
    }

    pub fn location(&self) -> Location {
        match &self.peeked {
            Some(Some(Ok(t))) => t.location,
            Some(Some(Err(e))) => e.location,
            _ => self.lexer.location(),
        }
    }
}
