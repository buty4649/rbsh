use super::{Annotate, Location, ParseError, Token, TokenKind};
use crate::token::TokenReader;

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
    string: String,
    kind: WordKind,
    loc: Location,
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
        match token {
            Annotate {
                value: TokenKind::Word(s, k),
                loc,
            } => self.push_word(s, k, loc),
            _ => unimplemented![],
        }
    }

    pub fn is_empty(self) -> bool {
        self.list.is_empty()
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

pub fn parse_wordlist(tokens: &mut TokenReader) -> Result<WordList, ParseError> {
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
