use crate::{context::Context, status::ExitStatus, syscall};
use dirs::home_dir;
use std::path::PathBuf;

pub fn cd(_: &mut Context, args: &[String]) -> ExitStatus {
    let path = if args.is_empty() {
        home_dir().unwrap_or_default()
    } else {
        if args.len() >= 2 {
            eprintln!("rbsh: cd: too many arguments");
            return ExitStatus::failure();
        }
        let mut path = PathBuf::new();
        path.push(&*args[0]);
        path
    };

    match syscall::set_current_dir(path) {
        Ok(_) => ExitStatus::success(),
        Err(e) => {
            eprintln!("rbsh: cd: {e}");
            ExitStatus::failure()
        }
    }
}
