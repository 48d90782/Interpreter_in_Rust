use crate::lexer::Lexer;

mod token;
mod lexer;
mod constants;
mod repl;

fn main() {
    let input:String = "5".to_string();

    let mut l = Lexer::new(input);
    l.read_char();
    let tok = l.next_token();
    println!("literal is: {}, token type is {}", tok.literal, tok.r#type);

    println!("Hello, world!");
}
