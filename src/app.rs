use super::{
    exec::syscall::{SysCallWrapper, Wrapper},
    parse_command_line,
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
        let context = Context::new();

        let mut rl = Editor::<()>::new();

        if self.tty_avaliable {
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
                        status = cmds.execute(&context).unwrap();
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
