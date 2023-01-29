use super::syscall::SysCallError;
use rbsh_parser::Location;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellErrorKind {
    SysCallError(String, nix::Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellError {
    kind: ShellErrorKind,
    loc: Location,
}

impl ShellError {
    pub fn new(kind: ShellErrorKind, loc: Location) -> Self {
        Self { kind, loc }
    }

    pub fn syscall_error(e: SysCallError, loc: Location) -> Self {
        Self::new(
            ShellErrorKind::SysCallError(e.name().to_string(), e.errno()),
            loc,
        )
    }
}
