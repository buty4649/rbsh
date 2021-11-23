use crate::{context::Context, status::ExitStatus, utils::Escape};
use clap::{App, AppSettings, Arg};

pub fn echo(_: &Context, args: &[String]) -> ExitStatus {
    let args = App::new("echo")
        .setting(AppSettings::NoBinaryName)
        .args(&[
            Arg::with_name("do_not_output_newline")
                .short("n")
                .multiple(true),
            Arg::with_name("escape").short("e").multiple(true),
            Arg::with_name("strings").multiple(true),
        ])
        .get_matches_from(args);

    let strings = args
        .values_of("strings")
        .map_or(vec![], |v| v.collect())
        .iter()
        .map(|s| match args.occurrences_of("escape") {
            0 => s.to_string(),
            _ => s.escape(),
        })
        .collect::<Vec<_>>();

    print!("{}", strings.join(" "));
    if args.occurrences_of("do_not_output_newline") == 0 {
        println!();
    }

    ExitStatus::success()
}
