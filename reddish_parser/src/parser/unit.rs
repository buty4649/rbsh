use crate::{Redirect, Word};

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
pub enum ConnecterKind {
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnitKind {
    SimpleCommand {
        command: Vec<Vec<Word>>,
        redirect: Option<Vec<Redirect>>,
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
        redirect: Option<Vec<Redirect>>,
    },
    Unless {
        condition: Box<Unit>,
        false_case: Vec<Unit>,
        true_case: Option<Vec<Unit>>,
        redirect: Option<Vec<Redirect>>,
    },
    While {
        condition: Box<Unit>,
        command: Vec<Unit>,
        redirect: Option<Vec<Redirect>>,
    },
    Until {
        condition: Box<Unit>,
        command: Vec<Unit>,
        redirect: Option<Vec<Redirect>>,
    },
    For {
        identifier: Vec<Word>,
        list: Option<Vec<Vec<Word>>>,
        command: Vec<Unit>,
        redirect: Option<Vec<Redirect>>,
    },
}
