mod echo;
mod read;

use super::{context::Context, status::ExitStatus};
use once_cell::sync::Lazy;

struct Builtin {
    name: &'static str,
    func: fn(&Context, &[String]) -> ExitStatus,
}

static BUILTIN: Lazy<Vec<Builtin>> = Lazy::new(|| {
    macro_rules! builtin {
        ($({ $name: expr, $func: path },)+) => {
            vec![$(Builtin {
                name: $name,
                func: $func,
            },)+]
        };
    }
    builtin![
        {"echo", echo::echo},
        {"read", read::read},
    ]
});

fn find_builtin_command(name: &str) -> Option<&Builtin> {
    BUILTIN.iter().find(|b| b.name == name)
}

pub fn is_builtin_command<T: AsRef<str>>(name: T) -> bool {
    find_builtin_command(name.as_ref()).is_some()
}

pub fn builtin_command_exec(ctx: &Context, command: String, args: &[String]) -> ExitStatus {
    match find_builtin_command(&*command) {
        None => {
            eprintln!("builtin: {} is not a shell builtin", command);
            ExitStatus::failure()
        }
        Some(b) => (b.func)(ctx, args),
    }
}
