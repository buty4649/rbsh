use super::{
    token::TokenReader,
    {Token, TokenKind},
};
use crate::{location::Location, Result};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WordKind {
    Normal,    // word
    Quote,     // "word"
    Literal,   // 'word'
    Command,   // `word`
    Variable,  // $word
    Parameter, // ${word}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Word {
    pub string: String,
    pub kind: WordKind,
    pub loc: Location,
}

impl Word {
    pub fn new(string: String, kind: WordKind, loc: Location) -> Self {
        Word { string, kind, loc }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct WordList {
    list: Vec<Word>,
}

impl WordList {
    pub fn new() -> Self {
        WordList { list: vec![] }
    }

    pub fn push(&mut self, word: Word) {
        self.list.push(word)
    }

    pub fn push_word(&mut self, string: String, kind: WordKind, loc: Location) {
        let word = Word::new(string, kind, loc);
        self.push(word)
    }

    pub fn push_word_token(&mut self, token: Token) {
        match token.value {
            TokenKind::Word(s, k) => self.push_word(s, k, token.location),
            _ => unimplemented![],
        }
    }

    pub fn first(&self) -> Word {
        self.list.first().unwrap().clone()
    }

    pub fn to_vec(&self) -> Vec<Word> {
        self.list.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}

impl Default for WordList {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Vec<Word>> for WordList {
    fn from(words: Vec<Word>) -> Self {
        let mut wordlist = Self::new();
        words.iter().for_each(|w| wordlist.push(w.clone()));
        wordlist
    }
}

impl From<Vec<Token>> for WordList {
    fn from(tokens: Vec<Token>) -> Self {
        let mut wordlist = Self::new();
        tokens
            .iter()
            .for_each(|t| wordlist.push_word_token(t.clone()));
        wordlist
    }
}

impl From<Vec<&str>> for WordList {
    fn from(input: Vec<&str>) -> Self {
        let mut wordlist = Self::new();
        let mut pos = 1;
        for s in input {
            wordlist.push_word(s.to_string(), WordKind::Normal, Location::new(pos, 1));
            pos += s.len();
        }
        wordlist
    }
}

pub fn parse_wordlist(tokens: &mut TokenReader) -> Result<WordList> {
    let mut result = WordList::new();

    loop {
        match tokens.peek_token() {
            Some(TokenKind::Word(_, _)) => {
                let token = tokens.next().unwrap();
                result.push_word_token(token)
            }
            Some(TokenKind::Space) => {
                tokens.next();
                break;
            }
            _ => break,
        }
    }
    Ok(result)
}

#[cfg(test)]
mod utils {
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
}