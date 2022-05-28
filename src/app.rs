use crate::{
    context::Context,
    exec::Executor,
    read_line::{
        ReadFromFile, ReadFromStdin, ReadFromString, ReadFromTTY, ReadLine, ReadLineError,
    },
    signal::{ignore_tty_signals, recognize_sigpipe},
    status::ExitStatus,
    syscall::isatty,
    Config, APP_NAME, VERSION,
};
use clap::Parser;
use reddish_parser::{parse_command_line, ErrorKind};
use std::{io, path::Path};

enum InputSource {
    Tty,
    Stdin,
    File(String),
    Command(String),
}

#[derive(Parser, Debug)]
#[clap(
    name = APP_NAME,
    version = VERSION,
    about = "Ruby-powerd shell",
)]
struct ReddishOptions {
    #[clap(short)]
    command: Option<String>,

    parameters: Vec<String>,
}

struct AppParameter {
    source: InputSource,
    positional_parameters: Vec<String>,
}

pub struct App {
    config: Config,
    ctx: Context,
}

impl App {
    pub fn run(args: Vec<String>) -> i32 {
        match Self::new() {
            Ok(mut app) => app.exec(args),
            Err(e) => {
                eprintln!("error: {}", e);
                1
            }
        }
    }

    fn new() -> Result<Self, std::io::Error> {
        let config = Config::new();
        let ctx = Context::new();
        Ok(Self { config, ctx })
    }

    fn parse_args(&self, args: Vec<String>) -> Result<AppParameter, io::Error> {
        let my_name = args.first().unwrap().to_owned();
        let opts = ReddishOptions::parse_from(args);
        let mut positional_parameters = opts.parameters.clone();

        let source = match opts.command {
            Some(command) => InputSource::Command(command),
            None => {
                if let Some(file) = positional_parameters.first() {
                    InputSource::File(file.to_owned())
                } else {
                    positional_parameters.push(my_name);
                    match self.isatty() {
                        true => InputSource::Tty,
                        false => InputSource::Stdin,
                    }
                }
            }
        };

        Ok(AppParameter {
            source,
            positional_parameters,
        })
    }

    fn isatty(&self) -> bool {
        isatty(0).unwrap_or(false)
    }

    fn exec(&mut self, args: Vec<String>) -> i32 {
        let params = match self.parse_args(args) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("reddish: {}", e);
                return ExitStatus::failure().code();
            }
        };
        self.set_shell_variables(&params);

        let mut rl: Box<dyn ReadLine> = match &params.source {
            InputSource::Tty => Box::new(ReadFromTTY::new()),
            InputSource::Stdin => Box::new(ReadFromStdin::new()),
            InputSource::File(path) => {
                let path = Path::new(&*path);
                let file = match ReadFromFile::new(path) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("reddish: {}", e);
                        return ExitStatus::failure().code();
                    }
                };
                Box::new(file)
            }
            InputSource::Command(command) => Box::new(ReadFromString::new(command)),
        };

        if let Some(e) = rl.load_history(self.config.history_file_path()).err() {
            eprintln!("reddish: load history error: {:?}", e);
        }

        // Ignore SIGPIPE by default
        // https://github.com/rust-lang/rust/pull/13158
        recognize_sigpipe().unwrap();

        if self.isatty() {
            ignore_tty_signals().unwrap();
        }

        let mut executor = match Executor::new() {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error: {}", e);
                return ExitStatus::failure().code();
            }
        };
        let mut cmdline = String::new();
        let mut linenumber = 1;
        loop {
            executor.reap_job();
            let prompt = match cmdline.is_empty() {
                true => self.ctx.get_var_or_default("PS1", "$ ".to_string()),
                false => self.ctx.get_var_or_default("PS2", "> ".to_string()),
            };
            match rl.readline(&prompt) {
                Ok(line) => {
                    cmdline.push_str(&line);
                    match parse_command_line(&cmdline, linenumber) {
                        Ok(cmds) => {
                            if !cmds.ignore_history && rl.add_history_entry(&cmdline) {
                                if let Some(e) =
                                    rl.save_history(self.config.history_file_path()).err()
                                {
                                    eprintln!("reddish: save history error: {:?}", e)
                                }
                            }
                            for cmd in cmds.to_vec() {
                                executor.execute_command(&mut self.ctx, cmd, None);
                            }

                            if rl.keep_linenumer() {
                                linenumber += cmdline.split('\n').count();
                            }

                            cmdline.clear()
                        }
                        Err(e) => {
                            match e.value {
                                ErrorKind::Eof => cmdline.push('\n'), // next line
                                _ => eprintln!("Error: {:?}", e),
                            }
                        }
                    }
                }
                Err(ReadLineError::Interrupted) => cmdline.clear(),
                Err(ReadLineError::Eof) => {
                    if cmdline.is_empty() {
                        break;
                    } else {
                        cmdline.clear()
                    }
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }

        executor.close();
        self.ctx.status.code()
    }

    fn set_shell_variables(&mut self, params: &AppParameter) {
        macro_rules! vars {
            ($({$name: tt, $var: expr}, )+) => {
                $(
                    self.ctx.set_var(stringify!($name), $var);
                )+
            };
        }

        vars![
            {PS1, "reddish> "},
            {PS2, "> "},
            {IFS, " \t\n"},
        ];

        self.ctx.positional_parameters = params.positional_parameters.clone();
    }
}
