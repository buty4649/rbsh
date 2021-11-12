use super::error::ShellError;
use nix::sys::signal::Signal;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ExitStatus {
    code: i32,
}

impl ExitStatus {
    pub fn new(code: i32) -> Self {
        ExitStatus { code }
    }

    pub fn success() -> Self {
        Self::new(0)
    }

    pub fn failure() -> Self {
        Self::new(1)
    }

    pub fn code(self) -> i32 {
        self.code
    }

    pub fn is_success(self) -> bool {
        self.code == 0
    }

    pub fn is_error(self) -> bool {
        !self.is_success()
    }

    pub fn signaled(sig: Signal) -> Self {
        Self::new(128 + sig as i32)
    }
}

impl Default for ExitStatus {
    fn default() -> Self {
        Self::new(0)
    }
}

pub type Result<T> = std::result::Result<T, ShellError>;
