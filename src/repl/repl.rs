use std::io::{self, Write};

use crate::lexer::{lexer::Lexer, token::Token};

const PROMPT: &str = ">> ";

pub fn start() {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut lexer = Lexer::new(input);
        loop {
            let token = lexer.next_token();
            if token == Token::Eof {
                break;
            }
            print!("{:?} ", token);
        }
        println!();
    }
}
