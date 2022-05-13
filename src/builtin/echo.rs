use crate::{context::Context, status::ExitStatus, utils::Escape};
use clap::{Arg, Command};

pub fn echo(_: &mut Context, args: &[String]) -> ExitStatus {
    let args = Command::new("echo")
        .no_binary_name(true)
        .args(&[
            Arg::new("do_not_output_newline")
                .short('n')
                .multiple_values(true),
            Arg::new("escape").short('e').multiple_values(true),
            Arg::new("strings").multiple_values(true),
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
