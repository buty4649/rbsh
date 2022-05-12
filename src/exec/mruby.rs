use crate::status::ExitStatus;
use clap::{Arg, ArgMatches, Command, Result as ClapResult};
use rust_mruby::MRuby;
use std::{fs::File, io, io::Read};

pub fn mruby_exec(mrb: &MRuby, args: &[String]) -> ExitStatus {
    let args = match parse_args(args) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("{}", e);
            return ExitStatus::failure();
        }
    };

    let mut option = match args.values_of("option") {
        Some(v) => v.map(|v| v.to_string()).collect::<Vec<_>>(),
        None => vec![],
    };

    let status = match args.value_of("command") {
        Some(command) => mrb.exec_from_string(command, &option, None),
        None => match option.is_empty() {
            true => {
                let command = match read_from_stdin() {
                    Ok(s) => s,
                    Err(e) => {
                        if e.kind() != io::ErrorKind::Interrupted {
                            eprintln!("mruby: {}", e);
                        }
                        return ExitStatus::failure();
                    }
                };
                mrb.exec_from_string(command, &option, None)
            }
            false => {
                let filename = option.remove(0);
                let mut file = match File::open(&*filename) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("mruby: {}", e);
                        return ExitStatus::failure();
                    }
                };
                let mut command = String::new();
                if let Some(e) = file.read_to_string(&mut command).err() {
                    eprintln!("mruby: {}", e);
                    return ExitStatus::failure();
                }

                mrb.exec_from_string(command, &option, Some(&filename))
            }
        },
    };

    match status {
        Ok(_) => ExitStatus::success(),
        Err(e) => {
            eprintln!("{}", e);
            ExitStatus::failure()
        }
    }
}

fn read_from_stdin() -> Result<String, io::Error> {
    let mut stdin = io::stdin();
    let mut result = vec![];

    loop {
        let mut buf = [0u8; 1024];
        match stdin.read(&mut buf) {
            Ok(s) if s == 0 => break,
            Ok(s) => result.extend_from_slice(&buf[0..s]),
            Err(e) => return Err(e),
        }
    }

    let s = std::str::from_utf8(&result).unwrap();
    Ok(s.to_string())
}

fn parse_args(args: &[String]) -> ClapResult<ArgMatches> {
    Command::new("mruby")
        .about("Run the internal mruby")
        .no_binary_name(true)
        .arg(Arg::new("command").short('e').takes_value(true))
        .arg(Arg::new("option").multiple_values(true))
        .help_template(
            "{bin} - {about}

USAGE:
    {usage}

{all-args}",
        )
        .try_get_matches_from(args)
}
