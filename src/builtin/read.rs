use crate::{
    context::Context,
    exec::syscall::{SysCallWrapper, Wrapper},
    signal::change_sa_restart_flag,
    status::ExitStatus,
};
use clap::{App, AppSettings, Arg, ArgMatches, Result as ClapResult};
use std::{io, os::unix::io::RawFd};

const DEFAULT_IFS: &str = " \t\n";

pub fn read(ctx: &Context, args: &[String]) -> ExitStatus {
    let args = match parse_args(args) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("{}", e);
            return ExitStatus::failure();
        }
    };

    let fd = match args.value_of("fd") {
        Some(fd) => match fd.parse::<RawFd>() {
            Ok(fd) => fd,
            Err(e) => {
                eprintln!("read: {}", e);
                return ExitStatus::failure();
            }
        },
        None => unreachable![],
    };
    let mut input = String::new();
    let status = match readline(ctx.wrapper(), fd, &mut input) {
        Err(e) => {
            if e.kind() != io::ErrorKind::Interrupted {
                eprintln!("{}", e);
            }
            ExitStatus::failure()
        }
        Ok(s) => {
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

fn parse_args<'a>(args: &[String]) -> ClapResult<ArgMatches<'a>> {
    App::new("read")
        .about("read from input")
        .after_help("")
        .setting(AppSettings::NoBinaryName)
        .arg(
            Arg::with_name("fd")
                .short("u")
                .takes_value(true)
                .default_value("0"),
        )
        .arg(Arg::with_name("name").multiple(true))
        .template(
            "{bin} - {about}

USAGE:
    {usage}

{all-args}",
        )
        .get_matches_from_safe(args)
}

fn readline(wrapper: &Wrapper, fd: RawFd, output: &mut String) -> Result<usize, io::Error> {
    change_sa_restart_flag(false)?;
    let mut size = 0;
    let mut buf = [0u8; 1024];
    let result = match wrapper.read(fd, &mut buf) {
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
