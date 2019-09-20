use crate::lexer::Lexer;

mod token;
mod lexer;
mod constants;

fn main() {
    let input:String = "let a".to_string();

    let mut l = Lexer::new(input);
    l.read_char();
    l.next_token();

    println!("Hello, world!");
}
