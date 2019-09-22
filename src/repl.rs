use std::io::{self, Write};
use crate::lexer::Lexer;
use crate::constants::EOF;

pub fn start() {
    loop {
        let mut input = String::new();
        println!(">> ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut input).expect("Error reading line!");

        if input == "exit" {
            break
        }
        let mut l = Lexer::new(input);

        loop {
            let tok = l.next_token();
            if tok.r#type == EOF {
                break
            }
            println!("token type is: {}, literal is: {}", tok.r#type, tok.literal);
        }
    }
}