use super::{
    context::Context,
    error::ShellErrorKind,
    exec::{
        syscall::{SysCallWrapper, Wrapper},
        Executor,
    },
    parse_command_line,
    read_line::{ReadFromFile, ReadFromTTY, ReadLine, ReadLineError},
    signal::{ignore_tty_signals, recognize_sigpipe},
    status::ExitStatus,
    Config, APP_NAME, VERSION,
};
use std::path::PathBuf;

pub struct App {
    config: Config,
    ctx: Context,
}

impl<'a> App {
    pub fn run(args: Vec<String>) -> i32 {
        let wrapper = Wrapper::new();
        match Self::new(wrapper) {
            Ok(mut app) => app.exec(args),
            Err(e) => {
                eprintln!("error: {}", e);
                1
            }
        }
    }

    fn new(wrapper: Wrapper) -> Result<Self, std::io::Error> {
        let config = Config::new();
        let ctx = Context::new(wrapper.clone());
        Ok(Self { config, ctx })
    }

    fn parse_args(&self, args: Vec<String>) -> clap::ArgMatches<'a> {
        clap::App::new(APP_NAME)
            .bin_name("reddish")
            .version(VERSION)
            .about("Ruby-powerd shell.")
            .arg(clap::Arg::with_name("script-file").index(1))
            .arg(clap::Arg::with_name("option").multiple(true))
            .get_matches_from(args)
    }

    fn exec(&mut self, args: Vec<String>) -> i32 {
        self.ctx.set_var("0", &args[0].to_string());
        self.ctx.set_var("PS1", "reddish> ");
        self.ctx.set_var("PS2", "> ");

        let args = self.parse_args(args);

        // Ignore SIGPIPE by default
        // https://github.com/rust-lang/rust/pull/13158
        recognize_sigpipe(self.ctx.wrapper()).unwrap();

        let isatty = self.ctx.wrapper().isatty(0).unwrap_or(false);
        if isatty {
            ignore_tty_signals(self.ctx.wrapper()).unwrap();
        }

        let mut rl: Box<dyn ReadLine> = match args.value_of("script-file") {
            Some(file) => {
                let mut p = PathBuf::new();
                p.push(file);
                let file = match ReadFromFile::new(self.ctx.wrapper(), Some(p)) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!("{:?}", e);
                        return ExitStatus::failure().code();
                    }
                };
                Box::new(file)
            }
            None => match isatty {
                true => Box::new(ReadFromTTY::new()),
                false => {
                    let file = match ReadFromFile::new(self.ctx.wrapper(), None) {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!("{:?}", e);
                            return ExitStatus::failure().code();
                        }
                    };
                    Box::new(file)
                }
            },
        };

        let mut path = PathBuf::new();
        path.push(self.config.history_file());
        if let Some(e) = rl.load_history(path).err() {
            eprintln!("load history error: {:?}", e);
        }

        let status = ExitStatus::new(0);
        let mut executor = match Executor::new(&self.ctx) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error: {}", e);
                return ExitStatus::failure().code();
            }
        };
        let mut cmdline = String::new();
        loop {
            executor.reap_job();
            let prompt = match cmdline.is_empty() {
                true => self.ctx.get_var_or_default("PS1", "$ ".to_string()),
                false => self.ctx.get_var_or_default("PS2", "> ".to_string()),
            };
            match rl.readline(&prompt) {
                Ok(line) => {
                    cmdline.push_str(&line);
                    match parse_command_line(&cmdline) {
                        Ok(cmds) => {
                            if !cmds.ignore_history() {
                                rl.add_history_entry(&cmdline);
                            }
                            for cmd in cmds.to_vec() {
                                executor.execute_command(cmd, None);
                            }
                            cmdline = String::new()
                        }
                        Err(e) => {
                            match e.value() {
                                ShellErrorKind::Eof => cmdline.push('\n'), // next line
                                _ => eprintln!("Error: {:?}", e),
                            }
                        }
                    }
                }
                Err(ReadLineError::Interrupted) => (),
                Err(ReadLineError::Eof) => {
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }

        status.code()
    }
}
