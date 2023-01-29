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

#[cfg(test)]
mod test {
    use super::*;
    use crate::{location, Error, WordKind};
    use std::any::type_name;

    #[test]
    fn lexer_iter() {
        fn name<T>(_: T) -> &'static str {
            type_name::<T>()
        }

        let iter = Lexer::new("abc", 0).iter();
        assert_eq!(name(iter), "rbsh_parser::lexer::iterator::LexerIterator");
    }

    #[test]
    fn next() {
        let mut iter = Lexer::new("abc", 0).iter();
        assert_eq!(
            iter.next(),
            Some(Ok(Token::word("abc", WordKind::Normal, location!())))
        );
        assert_eq!(iter.next(), None);

        let mut iter = Lexer::new("'abc", 0).iter();
        assert_eq!(
            iter.next(),
            Some(Err(Error::unterminated_string(location!())))
        );
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn peek() {
        let mut iter = Lexer::new("abc", 0).iter();
        assert_eq!(
            iter.peek(),
            Some(&Ok(Token::word("abc", WordKind::Normal, location!())))
        );
        assert_eq!(
            iter.peek(),
            Some(&Ok(Token::word("abc", WordKind::Normal, location!())))
        );
        iter.next();
        assert_eq!(iter.peek(), None);

        let mut iter = Lexer::new("'abc", 0).iter();
        assert_eq!(
            iter.peek(),
            Some(&Err(Error::unterminated_string(location!())))
        );
    }

    #[test]
    fn skipe_if_space() {
        let mut iter = Lexer::new(" abc", 0).iter();
        assert_eq!(iter.skip_if_space(), Ok(true));
        assert_eq!(iter.skip_if_space(), Ok(false));
        assert_eq!(
            iter.next(),
            Some(Ok(Token::word("abc", WordKind::Normal, location!(2))))
        );
        assert_eq!(iter.skip_if_space(), Ok(false));

        let mut iter = Lexer::new("'abc", 0).iter();
        assert_eq!(
            iter.skip_if_space(),
            Err(Error::unterminated_string(location!()))
        );
    }

    #[test]
    fn skip_if_space_or_newline() {
        let mut iter = Lexer::new(" abc", 0).iter();
        assert_eq!(iter.skip_if_space_or_newline(), Ok(true));
        assert_eq!(iter.skip_if_space_or_newline(), Ok(false));
        assert_eq!(
            iter.next(),
            Some(Ok(Token::word("abc", WordKind::Normal, location!(2))))
        );
        assert_eq!(iter.skip_if_space_or_newline(), Ok(false));

        let mut iter = Lexer::new("\nabc", 0).iter();
        assert_eq!(iter.skip_if_space_or_newline(), Ok(true));
        assert_eq!(iter.skip_if_space_or_newline(), Ok(false));
        assert_eq!(
            iter.next(),
            Some(Ok(Token::word("abc", WordKind::Normal, location!(1, 2))))
        );
        assert_eq!(iter.skip_if_space_or_newline(), Ok(false));

        let mut iter = Lexer::new("'abc", 0).iter();
        assert_eq!(
            iter.skip_if_space_or_newline(),
            Err(Error::unterminated_string(location!()))
        );
    }

    #[test]
    fn location() {
        let mut iter = Lexer::new("abc", 0).iter();
        assert_eq!(iter.location(), location!());

        assert_eq!(iter.skip_if_space(), Ok(false));
        assert_eq!(iter.location(), location!());

        iter.next();
        assert_eq!(iter.location(), location!(4));

        let mut iter = Lexer::new("'abc", 0).iter();
        assert_eq!(
            iter.peek(),
            Some(&Err(Error::unterminated_string(location!())))
        );
        assert_eq!(iter.location(), location!());
    }
}
