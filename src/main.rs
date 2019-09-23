use crate::repl::start;

mod token;
mod lexer;
mod repl;
mod ast;
mod parser;

fn main() {
    start()
}
