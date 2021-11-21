use crate::{
    context::Context,
    error::ShellErrorKind,
    exec::{
        syscall::{SysCallWrapper, Wrapper},
        Executor,
    },
    parse_command_line,
    read_line::{
        ReadFromFile, ReadFromStdin, ReadFromString, ReadFromTTY, ReadLine, ReadLineError,
    },
    signal::{ignore_tty_signals, recognize_sigpipe},
    status::ExitStatus,
    Config, APP_NAME, VERSION,
};
use std::{io, path::Path};

enum InputSource {
    Tty,
    Stdin,
    File(String),
    Command(String),
}

struct AppParameter {
    source: InputSource,
    bin_name: String,
    positional_parameters: Vec<String>,
}

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
        let ctx = Context::new(wrapper);
        Ok(Self { config, ctx })
    }

    fn parse_args(&self, args: Vec<String>) -> Result<AppParameter, io::Error> {
        let mut bin_name = args[0].to_string();
        let args = &clap::App::new(APP_NAME)
            .bin_name("reddish")
            .version(VERSION)
            .about("Ruby-powerd shell.")
            .arg(clap::Arg::with_name("command").short("c").takes_value(true))
            .arg(clap::Arg::with_name("option").multiple(true))
            .get_matches_from(args);

        let command = args.value_of("command");
        let mut option = match args.values_of("option") {
            Some(v) => v.collect::<Vec<_>>(),
            None => vec![],
        }
        .into_iter();

        let source = match command {
            Some(command) => {
                if let Some(name) = option.next() {
                    bin_name = name.to_string();
                }
                InputSource::Command(command.to_string())
            }
            None => {
                if let Some(file) = option.next() {
                    InputSource::File(file.to_string())
                } else {
                    match self.isatty() {
                        true => InputSource::Tty,
                        false => InputSource::Stdin,
                    }
                }
            }
        };

        let positional_parameters = option.map(|opt| opt.to_string()).collect::<Vec<_>>();
        Ok(AppParameter {
            source,
            bin_name,
            positional_parameters,
        })
    }

    fn isatty(&self) -> bool {
        self.ctx.wrapper().isatty(0).unwrap_or(false)
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
                let file = match ReadFromFile::new(self.ctx.wrapper(), path) {
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
        recognize_sigpipe(self.ctx.wrapper()).unwrap();

        if self.isatty() {
            ignore_tty_signals(self.ctx.wrapper()).unwrap();
        }

        let mut executor = match Executor::new(&self.ctx) {
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
                            if !cmds.ignore_history() && rl.add_history_entry(&cmdline) {
                                if let Some(e) =
                                    rl.save_history(self.config.history_file_path()).err()
                                {
                                    eprintln!("reddish: save history error: {:?}", e)
                                }
                            }
                            for cmd in cmds.to_vec() {
                                executor.execute_command(cmd, None);
                            }

                            if rl.keep_linenumer() {
                                linenumber += cmdline.split('\n').count();
                            }

                            cmdline.clear()
                        }
                        Err(e) => {
                            match e.value() {
                                ShellErrorKind::Eof => cmdline.push('\n'), // next line
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
        self.ctx.get_status().code()
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

        self.ctx.set_bin_name(params.bin_name.to_string());
        self.ctx
            .set_positional_parameters(&params.positional_parameters)
    }
}
