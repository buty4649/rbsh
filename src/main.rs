use anyhow::Result;
use reddish::{config::Config, context::Context, exec::ShellExecute, parser::parse_command_line};
use rustyline::{error::ReadlineError, Editor};

fn main() -> Result<()> {
    let config = Config::new();
    let context = Context::new();

    let mut rl = Editor::<()>::new();
    rl.load_history(&*config.history_file()).unwrap_or_default();

    loop {
        let readline = rl.readline("reddish> ");
        match readline {
            Ok(line) => match parse_command_line(line.as_str()) {
                Ok(cmds) => {
                    if !cmds.ignore_history() {
                        rl.add_history_entry(line.as_str());
                    }
                    cmds.execute(&context).unwrap();
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

    rl.save_history(&*config.history_file())?;
    Ok(())
}
