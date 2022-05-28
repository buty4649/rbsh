use reddish_parser::parse_command_line;
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    loop {
        if input.is_empty() {
            print!("$ ");
        } else {
            print!("> ")
        }
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Err(_) => break,
            Ok(n) if n == 0 => break,
            Ok(_) => {
                let buffer = buffer.trim_end();
                if buffer.is_empty() {
                    match parse_command_line(&input, 0) {
                        Ok(_) => (),
                        Err(e) => eprintln!("Error: {:?}", e),
                    }
                    input.clear();
                } else {
                    if !input.is_empty() {
                        input.push('\n');
                    }
                    input.push_str(buffer);
                }
            }
        }
    }
}
