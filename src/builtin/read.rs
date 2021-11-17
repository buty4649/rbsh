use crate::{context::Context, status::ExitStatus};
use clap::{App, AppSettings, Arg};
use std::io;

const DEFAULT_IFS: &str = " \t\n";

pub fn read(ctx: &Context, args: &[String]) -> ExitStatus {
    let args = App::new("read")
        .setting(AppSettings::NoBinaryName)
        .args(&[Arg::with_name("name").multiple(true)])
        .get_matches_from(args);

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Err(e) => {
            eprintln!("{}", e);
            ExitStatus::failure()
        }
        Ok(s) if s == 0 => ExitStatus::failure(),
        Ok(_) => {
            let names = args
                .values_of("name")
                .map_or(vec!["REPLY"], |v| v.collect());

            let ifs = ctx
                .get_var("IFS")
                .unwrap_or_else(|| DEFAULT_IFS.to_string());
            let pat = ifs.chars().collect::<Vec<_>>();
            let mut vars = input.trim_end_matches('\n').splitn(names.len(), &pat[..]);
            for name in names {
                let val = vars.next().unwrap_or("");
                ctx.set_var(name, val);
            }

            ExitStatus::success()
        }
    }
}
