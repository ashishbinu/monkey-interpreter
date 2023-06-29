mod lexer;
mod repl;

use crate::repl::repl::start;

fn main() {
    println!("Monkey Interpreter");
    start();
}
