use super::{
    exec::syscall::{SysCallWrapper, Wrapper},
    parse_command_line,
    signal::{ignore_tty_signals, recognize_sigpipe},
    status::ExitStatus,
    Config, Context, ShellExecute, APP_NAME, VERSION,
};
use rustyline::{error::ReadlineError, Editor};

pub struct App<'a, 'b> {
    app: clap::App<'a, 'b>,
    tty_avaliable: bool,
}

impl<'a, 'b> App<'a, 'b> {
    pub fn new() -> Self {
        Self::new_at(Wrapper::new())
    }

    fn new_at(wrapper: Wrapper) -> Self {
        let tty_avaliable = wrapper.isatty(0).unwrap_or(false);
        let app = clap::App::new(APP_NAME)
            .version(VERSION)
            .about("Ruby-powerd shell.");
        Self { app, tty_avaliable }
    }

    pub fn run(self, args: Vec<String>) -> i32 {
        self.app.get_matches_from(args);

        let config = Config::new();
        let mut ctx = Context::new();

        let mut rl = Editor::<()>::new();

        // Ignore SIGPIPE by default
        // https://github.com/rust-lang/rust/pull/13158
        recognize_sigpipe(&ctx).unwrap();

        if self.tty_avaliable {
            ignore_tty_signals(&ctx).unwrap();
            rl.load_history(&*config.history_file()).unwrap_or_default();
        }

        let mut status = ExitStatus::new(0);
        loop {
            let readline = rl.readline("reddish> ");
            match readline {
                Ok(line) => match parse_command_line(line.as_str()) {
                    Ok(cmds) => {
                        if !cmds.ignore_history() {
                            rl.add_history_entry(line.as_str());
                        }
                        status = cmds.execute(&mut ctx).unwrap();
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
            rl.save_history(&*config.history_file()).unwrap();
        }

        status.code()
    }
}
