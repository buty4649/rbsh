use super::error::ShellError;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ExitStatus {
    code: i32,
}

impl ExitStatus {
    pub fn new(code: i32) -> Self {
        ExitStatus { code }
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
}

pub type Result<T> = std::result::Result<T, ShellError>;
