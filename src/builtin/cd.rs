use crate::{context::Context, exec::syscall::SysCallWrapper, status::ExitStatus};
use dirs::home_dir;
use std::path::PathBuf;

pub fn cd(ctx: &Context, args: &[String]) -> ExitStatus {
    let path = if args.is_empty() {
        home_dir().unwrap_or_default()
    } else {
        if args.len() >= 2 {
            eprintln!("reddish: cd: too many arguments");
            return ExitStatus::failure();
        }
        let mut path = PathBuf::new();
        path.push(&*args[0]);
        path
    };

    match ctx.wrapper().set_current_dir(path) {
        Ok(_) => ExitStatus::success(),
        Err(e) => {
            eprintln!("reddish: cd: {}", e);
            ExitStatus::failure()
        }
    }
}