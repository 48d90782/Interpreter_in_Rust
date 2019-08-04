use crate::token::Token;
use crate::constants::{ASSIGN, SEMICOLON, LPAREN, RPAREN, COMMA, PLUS, LBRACE, RBRACE, EOF};
use std::borrow::Borrow;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
        }
    }

    /// read char
    ///
    ///
    ///
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }

        self.position = self.read_position;
        self.read_position = self.read_position + 1;
    }

    ///
    ///
    ///
    ///
    pub fn next_token(&mut self) -> Token {
        let mut token = Token::new();


        match self.ch as char {
            '=' => {
                token = Token::new_token(ASSIGN, self.ch);
            }

            ';' => {
                token = Token::new_token(SEMICOLON, self.ch);
            }
            '(' => {
                token = Token::new_token(LPAREN, self.ch);
            }
            ')' => {
                token = Token::new_token(RPAREN, self.ch);
            }
            ',' => {
                token = Token::new_token(COMMA, self.ch);
            }
            '+' => {
                token = Token::new_token(PLUS, self.ch);
            }
            '{' => {
                token = Token::new_token(LBRACE, self.ch);
            }
            '}' => {
                token = Token::new_token(RBRACE, self.ch);
            }
            '0' => {
                token = Token::new_token(EOF, 0);
            }

            _ => {
                if self.ch.is_ascii_alphabetic() {
//                    token = Token::new_token()
                }
            }
        }

        self.read_char();
        token
    }

    pub fn read_identifier(&mut self) -> &'a str {
        let position = self.position;
        loop {
            if self.ch.is_ascii_alphabetic() {
                self.read_char();
            } else {
                break;
            }
        }
        self.input.as_bytes()[position..self.position].
    }
}

//pub fn is_letter(par:u8) -> bool {
//    match *self {
//        b'A'..=b'Z' | b'a'..=b'z' => true,
//        _ => false
//    }
//
//}