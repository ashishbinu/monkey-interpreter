mod lexer;
mod repl;

use std::io;

use crate::repl::repl::start;

fn main() {
    println!("Monkey Interpreter");
    let mut stdout = io::stdout();
    let stdin = io::stdin();
    start(&mut stdin.lock(), &mut stdout);
}
