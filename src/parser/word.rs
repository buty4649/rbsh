use super::{peek_token, Annotate, Location, ParseError, Token, TokenKind};
use std::iter::Peekable;

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

pub fn parse_wordlist<T>(tokens: &mut Peekable<T>) -> Result<WordList, ParseError>
where
    T: Iterator<Item = Token>,
{
    let mut result = WordList::new();

    loop {
        match peek_token(tokens) {
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
