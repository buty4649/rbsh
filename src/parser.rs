pub mod redirect;
pub mod token;
pub mod word;

mod command;
mod lexer;

use super::Result;
use command::{parse_command, ConnecterKind};
use lexer::lex;
use redirect::RedirectList;
use std::iter::Iterator;
use token::{Token, TokenKind, TokenReader};
use word::{parse_wordlist, Word, WordList};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommandList {
    list: Vec<UnitKind>,
    ignore_history: bool,
    current: usize,
}

impl CommandList {
    pub fn new(list: Vec<UnitKind>, ignore_history: bool) -> Self {
        Self {
            list,
            ignore_history,
            current: 0,
        }
    }
}

impl Iterator for CommandList {
    type Item = UnitKind;

    fn next(&mut self) -> Option<UnitKind> {
        if self.current >= self.list.len() {
            None
        } else {
            let result = self.list[self.current].clone();
            self.current += 1;
            Some(result)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnitKind {
    SimpleCommand {
        command: Vec<WordList>,
        redirect: RedirectList,
        background: bool,
    },
    Connecter {
        left: Box<UnitKind>,
        right: Box<UnitKind>,
        kind: ConnecterKind,
        background: bool,
    },
    If {
        condition: Box<UnitKind>,
        true_case: Vec<UnitKind>,
        false_case: Option<Vec<UnitKind>>,
        redirect: RedirectList,
        background: bool,
    },
    Unless {
        condition: Box<UnitKind>,
        false_case: Vec<UnitKind>,
        true_case: Option<Vec<UnitKind>>,
        redirect: RedirectList,
        background: bool,
    },
    While {
        condition: Box<UnitKind>,
        command: Vec<UnitKind>,
        redirect: RedirectList,
        background: bool,
    },
    Until {
        condition: Box<UnitKind>,
        command: Vec<UnitKind>,
        redirect: RedirectList,
        background: bool,
    },
    For {
        identifier: Word,
        list: Option<Vec<WordList>>,
        command: Vec<UnitKind>,
        redirect: RedirectList,
        background: bool,
    },
}

pub fn parse_command_line(s: &str) -> Result<CommandList> {
    let tokens = lex(s)?;
    let mut tokens = TokenReader::new(tokens);
    let mut result = vec![];

    // If it starts with a Space, ignore the command history.
    let ignore_history = matches!(tokens.skip_space(), Some(_));

    loop {
        match parse_command(&mut tokens)? {
            None => break,
            Some(c) => result.push(c),
        }
    }

    let result = CommandList::new(result, ignore_history);
    Ok(result)
}
