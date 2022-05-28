mod debug;
mod redirect;
mod token;
mod word;

use command::parse_command;

mod command;

pub use command::ConnecterKind;
pub use redirect::{Redirect, RedirectKind, RedirectList};
pub use token::{Token, TokenKind};
pub use word::{parse_wordlist, Word, WordKind, WordList};

use crate::{lexer::Lexer, Result};
use token::TokenReader;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommandList {
    list: Vec<Unit>,
    ignore_history: bool,
    current: usize,
}

impl CommandList {
    pub fn new(list: Vec<Unit>, ignore_history: bool) -> Self {
        Self {
            list,
            ignore_history,
            current: 0,
        }
    }

    pub fn to_vec(&self) -> Vec<Unit> {
        self.list.clone()
    }

    pub fn ignore_history(&self) -> bool {
        self.ignore_history
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Unit {
    pub kind: UnitKind,
    pub background: bool,
}

impl Unit {
    pub fn new(kind: UnitKind, background: bool) -> Self {
        Self { kind, background }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnitKind {
    SimpleCommand {
        command: Vec<WordList>,
        redirect: RedirectList,
    },
    Connecter {
        left: Box<Unit>,
        right: Box<Unit>,
        kind: ConnecterKind,
    },
    Pipe {
        left: Box<Unit>,
        right: Box<Unit>,
        both: bool,
    },
    If {
        condition: Box<Unit>,
        true_case: Vec<Unit>,
        false_case: Option<Vec<Unit>>,
        redirect: RedirectList,
    },
    Unless {
        condition: Box<Unit>,
        false_case: Vec<Unit>,
        true_case: Option<Vec<Unit>>,
        redirect: RedirectList,
    },
    While {
        condition: Box<Unit>,
        command: Vec<Unit>,
        redirect: RedirectList,
    },
    Until {
        condition: Box<Unit>,
        command: Vec<Unit>,
        redirect: RedirectList,
    },
    For {
        identifier: Word,
        list: Option<Vec<WordList>>,
        command: Vec<Unit>,
        redirect: RedirectList,
    },
}

pub fn parse_command_line<S: AsRef<str>>(
    s: S,
    linenumber: usize,
    debug: bool,
) -> Result<CommandList> {
    let tokens = Lexer::new(s.as_ref(), linenumber, debug).lex()?;

    let mut tokens = TokenReader::new(tokens);
    let mut result = vec![];

    // If it starts with a Space, ignore the command history.
    let ignore_history = matches!(tokens.skip_space(false), Some(_));

    loop {
        match parse_command(&mut tokens)? {
            None => break,
            Some(c) => result.push(c),
        }
    }

    if debug {
        debug::print(&result);
    }

    let result = CommandList::new(result, ignore_history);

    Ok(result)
}
