extern crate rbsh_parser;

use rbsh_parser::parse;
use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
    let mut rl = Editor::<()>::new().unwrap();
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                let parse = parse(&line, true);
                println!("{:?}", parse);
            }
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
