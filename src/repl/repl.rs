use std::io::{BufRead, Write};

use crate::lexer::{lexer::Lexer, token::Token};

const PROMPT: &str = ">> ";

pub fn start(stdin: &mut dyn BufRead, stdout: &mut dyn Write) {
    loop {
        write!(stdout, "{}", PROMPT).unwrap();
        stdout.flush().unwrap();

        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                let mut lexer = Lexer::new(input);
                loop {
                    let token = lexer.next_token();
                    if token == Token::Eof {
                        break;
                    }
                    write!(stdout, "{:?} ", token).unwrap();
                }
                writeln!(stdout).unwrap();
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            }
        }
    }
}
