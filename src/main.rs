use crate::lexer::Lexer;

mod token;
mod lexer;
mod constants;

fn main() {
    let input:&str = ";";

    let mut l = Lexer::new(input);
    println!("{:?}", l.read_char());
    l.next_token();

    println!("Hello, world!");
}
