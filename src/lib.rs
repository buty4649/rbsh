mod config;
mod context;
mod exec;
mod location;
mod parser;
mod status;

pub mod error;
pub use config::Config;
pub use context::Context;
pub use exec::ShellExecute;
pub use parser::parse_command_line;

static APP_NAME: &str = "reddish";
