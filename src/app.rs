use super::{
    exec::{
        syscall::{SysCallWrapper, Wrapper},
        Executor,
    },
    parse_command_line,
    signal::{ignore_tty_signals, recognize_sigpipe},
    status::ExitStatus,
    Config, APP_NAME, VERSION,
};
use rustyline::{error::ReadlineError, Editor};

pub struct App {
    config: Config,
    executor: Executor,
    tty_avaliable: bool,
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
        let executor = Executor::new(wrapper.clone())?;
        let tty_avaliable = wrapper.isatty(0).unwrap_or(false);
        Ok(Self {
            config,
            executor,
            tty_avaliable,
        })
    }

    fn parse_args(&self, args: Vec<String>) -> clap::ArgMatches<'a> {
        clap::App::new(APP_NAME)
            .version(VERSION)
            .about("Ruby-powerd shell.")
            .get_matches_from(args)
    }

    fn exec(&mut self, args: Vec<String>) -> i32 {
        let _args = self.parse_args(args);

        let ctx = self.executor.context();
        let mut rl = Editor::<()>::new();

        // Ignore SIGPIPE by default
        // https://github.com/rust-lang/rust/pull/13158
        recognize_sigpipe(&ctx).unwrap();

        if self.tty_avaliable {
            ignore_tty_signals(&ctx).unwrap();
            rl.load_history(&*self.config.history_file())
                .unwrap_or_default();
        }

        let status = ExitStatus::new(0);
        loop {
            let readline = rl.readline("reddish> ");
            match readline {
                Ok(line) => match parse_command_line(line.as_str()) {
                    Ok(cmds) => {
                        if !cmds.ignore_history() {
                            rl.add_history_entry(line.as_str());
                        }
                        for cmd in cmds.to_vec() {
                            self.executor.execute_command(cmd);
                        }
                    }
                    Err(e) => eprintln!("Error: {:?}", e),
                },
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }

        if self.tty_avaliable {
            rl.save_history(&*self.config.history_file()).unwrap();
        }

        status.code()
    }
}
