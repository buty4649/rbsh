use super::{
    context::Context,
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
    ctx: Context,
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
        let ctx = Context::new(wrapper.clone());
        let tty_avaliable = wrapper.isatty(0).unwrap_or(false);
        Ok(Self {
            config,
            ctx,
            tty_avaliable,
        })
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

        let _args = self.parse_args(args);
        let mut rl = Editor::<()>::new();

        // Ignore SIGPIPE by default
        // https://github.com/rust-lang/rust/pull/13158
        recognize_sigpipe(self.ctx.wrapper()).unwrap();

        if self.tty_avaliable {
            ignore_tty_signals(self.ctx.wrapper()).unwrap();
            rl.load_history(&*self.config.history_file())
                .unwrap_or_default();
        }

        let status = ExitStatus::new(0);
        let mut executor = match Executor::new(&self.ctx) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error: {}", e);
                return ExitStatus::failure().code();
            }
        };
        loop {
            executor.reap_job();
            let prompt = self.ctx.get_var("PS1").unwrap_or_else(|| "$ ".to_string());
            let readline = rl.readline(&*prompt);
            match readline {
                Ok(line) => match parse_command_line(line.as_str()) {
                    Ok(cmds) => {
                        if !cmds.ignore_history() {
                            rl.add_history_entry(line.as_str());
                        }
                        for cmd in cmds.to_vec() {
                            executor.execute_command(cmd, None);
                        }
                    }
                    Err(e) => eprintln!("Error: {:?}", e),
                },
                Err(ReadlineError::Interrupted) => (),
                Err(ReadlineError::Eof) => {
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
