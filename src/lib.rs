mod app;
mod config;
mod context;
mod error;
mod exec;
mod location;
mod parser;
mod signal;
mod status;

pub use app::App;
pub use config::Config;
pub use context::Context;
pub use exec::ShellExecute;
pub use parser::parse_command_line;

use clap::crate_version;

static APP_NAME: &str = "reddish";
static VERSION: &str = crate_version!();
