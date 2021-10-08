use anyhow::Result;
use reddish_shell::parser::parse_command_line;
use rustyline::{error::ReadlineError, Editor};

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => match parse_command_line(line.as_str()) {
                Ok(cmd) => println!("{:?}", cmd),
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
    Ok(())
}
