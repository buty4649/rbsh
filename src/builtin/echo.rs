use crate::{context::Context, status::ExitStatus, utils::Escape};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    no_binary_name = true,
    disable_help_flag = true,
    disable_version_flag = true,
    allow_hyphen_values = true,
    trailing_var_arg = true
)]
struct EchoOptions {
    #[clap(short = 'n')]
    trim: bool,

    #[clap(short)]
    escape: bool,

    #[clap(index = 1)]
    strings: Vec<String>,
}

pub fn echo(_: &mut Context, args: &[String]) -> ExitStatus {
    let opts = EchoOptions::try_parse_from(args);

    match opts {
        Err(e) => {
            eprint!("{e}");
        }
        Ok(opts) => {
            let str = opts.strings.join(" ");
            let str = if opts.escape { str.escape() } else { str };
            let str = if opts.trim { str } else { str + "\n" };
            print!("{str}");
        }
    }

    ExitStatus::success()
}
