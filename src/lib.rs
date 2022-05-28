mod app;
mod builtin;
mod config;
mod context;
mod error;
mod exec;
mod mockable_syscall;
mod read_line;
mod signal;
mod status;
mod utils;

pub use app::App;
pub use config::Config;
pub use context::Context;

use clap::crate_version;
use mockall_double::double;

#[double]
use mockable_syscall::inner as syscall;

static APP_NAME: &str = "reddish";
static VERSION: &str = crate_version!();

#[macro_export]
macro_rules! debug {
    ($f:expr, $($arg:tt)*) => {
        if $f {
            eprint!("debug: ");
            eprintln!($($arg)*);
        }
    };
}
