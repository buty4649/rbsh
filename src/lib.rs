mod config;
mod context;
mod exec;
mod location;
mod parser;

pub mod error;
pub use config::Config;
pub use context::Context;
pub use exec::ShellExecute;
pub use parser::parse_command_line;

static APP_NAME: &str = "reddish";

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ExitStatus {
    code: i32,
}

impl ExitStatus {
    fn new(code: i32) -> Self {
        ExitStatus { code }
    }

    fn is_success(self) -> bool {
        self.code == 0
    }

    fn is_error(self) -> bool {
        !self.is_success()
    }
}

pub type Result<T> = std::result::Result<T, error::ShellError>;
