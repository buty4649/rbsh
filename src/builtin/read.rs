use crate::{context::Context, signal::change_sa_restart_flag, status::ExitStatus, syscall};
use clap::Parser;
use std::{io, os::unix::io::RawFd};

#[derive(Parser, Debug)]
#[clap(
    name = "read",
    about = "read from input",
    no_binary_name = true,
    allow_negative_numbers = true,
    help_template = "{bin} - {about}

USAGE:
    {usage}

{all-args}"
)]
struct ReadOptions {
    #[clap(short = 'u')]
    fd: Option<RawFd>,

    names: Vec<String>,
}

pub fn read(ctx: &mut Context, args: &[String]) -> ExitStatus {
    let opts = match ReadOptions::try_parse_from(args) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            return ExitStatus::failure();
        }
    };

    let fd = opts.fd.unwrap_or(0);
    let mut input = String::new();
    let status = match readline(fd, &mut input) {
        Err(e) => {
            if e.kind() != io::ErrorKind::Interrupted {
                eprintln!("{}", e);
            }
            ExitStatus::failure()
        }
        Ok(s) => {
            let names = if opts.names.is_empty() {
                vec!["REPLY"]
            } else {
                opts.names.iter().map(AsRef::as_ref).collect()
            };

            let ifs = ctx.get_var("IFS").unwrap_or_default();
            let pat = ifs.chars().collect::<Vec<_>>();
            let mut vars = input.trim_end_matches('\n').splitn(names.len(), &pat[..]);
            for name in names {
                let val = vars.next().unwrap_or("");
                ctx.set_var(name, val);
            }

            if s > 0 {
                ExitStatus::success()
            } else {
                ExitStatus::failure()
            }
        }
    };
    change_sa_restart_flag(true).unwrap();

    status
}

fn readline(fd: RawFd, output: &mut String) -> Result<usize, io::Error> {
    change_sa_restart_flag(false)?;
    let mut size = 0;
    let mut buf = [0u8; 1024];
    let result = match syscall::read(fd, &mut buf) {
        Err(e) => Err(io::Error::from_raw_os_error(e.code())),
        Ok(s) => {
            let u = std::str::from_utf8(&buf[..s]).unwrap();
            output.push_str(u);
            size += s;
            Ok(size)
        }
    };
    change_sa_restart_flag(true)?;

    result
}
