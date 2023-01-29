use std::os::unix::io::RawFd;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node {
    Command {
        name: Vec<WordKind>,
        args: Option<Vec<Vec<WordKind>>>,
        redirect: Option<Vec<RedirectKind>>,
        parameter: Option<Vec<Parameter>>,
    },
    VariableAssignment {
        body: Vec<Parameter>,
    },
    If {
        body: Condition,
        elif_body: Option<Vec<Condition>>,
        else_body: Option<Vec<Node>>,
        redirect: Option<Vec<RedirectKind>>,
    },
    Unless {
        body: Condition,
        else_body: Option<Vec<Node>>,
        redirect: Option<Vec<RedirectKind>>,
    },
    While {
        body: Condition,
        redirect: Option<Vec<RedirectKind>>,
    },
    Until {
        body: Condition,
        redirect: Option<Vec<RedirectKind>>,
    },
    For {
        ident: String,
        subject: Option<Vec<Vec<WordKind>>>,
        body: Vec<Node>,
        redirect: Option<Vec<RedirectKind>>,
    },
    Select {
        ident: String,
        subject: Option<Vec<Vec<WordKind>>>,
        body: Vec<Node>,
        redirect: Option<Vec<RedirectKind>>,
    },
    Case {
        word: Vec<WordKind>,
        pattern: Option<Vec<CasePattern>>,
        redirect: Option<Vec<RedirectKind>>,
    },
    Function {
        ident: String,
        body: Box<Node>,
        redirect: Option<Vec<RedirectKind>>,
    },
    Group {
        body: Vec<Node>,
        redirect: Option<Vec<RedirectKind>>,
    },
    Subshell {
        body: Vec<Node>,
        redirect: Option<Vec<RedirectKind>>,
    },
    And {
        left: Box<Node>,
        right: Box<Node>,
    },
    Or {
        left: Box<Node>,
        right: Box<Node>,
    },
    Pipe {
        left: Box<Node>,
        right: Box<Node>,
        both: bool,
    },
    InvertReturn {
        body: Option<Box<Node>>,
    },
    Background {
        left: Box<Node>,
        right: Option<Box<Node>>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Condition {
    pub test: Box<Node>,
    pub body: Vec<Node>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CasePattern {
    pub pattern: Vec<Vec<WordKind>>,
    pub body: Vec<Node>,
    pub next_action: CasePatternNextAction,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub value: Option<Vec<WordKind>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CasePatternNextAction {
    End,
    FallThrough,
    TestNext,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WordKind {
    Bare(String),
    Quote(Vec<WordKind>),
    CommandSubstitute(Vec<Node>),
    Parameter(String),
}

impl WordKind {
    pub fn bare<S: Into<String>>(inner: S) -> Self {
        Self::Bare(inner.into())
    }

    pub fn parameter<S: Into<String>>(inner: S) -> Self {
        Self::Parameter(inner.into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RedirectKind {
    ReadFrom(RawFd, Vec<WordKind>),
    WriteTo(RawFd, Vec<WordKind>, bool),
    WriteBoth(Vec<WordKind>),
    ReadCopy(RawFd, RawFd, bool),
    WriteCopy(RawFd, RawFd, bool),
    ReadClose(RawFd),
    WriteClose(RawFd),
    AppendTo(RawFd, Vec<WordKind>),
    AppendBoth(Vec<WordKind>),
    ReadWrite(RawFd, Vec<WordKind>),
    HereString(RawFd, Vec<WordKind>),
}
