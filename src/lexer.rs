use crate::token::Token;
use crate::constants::{ASSIGN, SEMICOLON, LPAREN, RPAREN, COMMA, PLUS, LBRACE, RBRACE, EOF, INT, ILLEGAL};
use std::borrow::Borrow;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
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
        self.read_position += 1;
    }

    ///
    ///
    ///
    ///
    pub fn next_token(&mut self) -> Token {
        let mut token = Token::new();


        match self.ch as char {
            '=' => {
                token = Token::new_token(ASSIGN.parse::<String>().unwrap(), self.ch);
            }
            ';' => {
                token = Token::new_token(SEMICOLON.parse::<String>().unwrap(), self.ch);
            }
            '(' => {
                token = Token::new_token(LPAREN.parse::<String>().unwrap(), self.ch);
            }
            ')' => {
                token = Token::new_token(RPAREN.parse::<String>().unwrap(), self.ch);
            }
            ',' => {
                token = Token::new_token(COMMA.parse::<String>().unwrap(), self.ch);
            }
            '+' => {
                token = Token::new_token(PLUS.parse::<String>().unwrap(), self.ch);
            }
            '{' => {
                token = Token::new_token(LBRACE.parse::<String>().unwrap(), self.ch);
            }
            '}' => {
                token = Token::new_token(RBRACE.parse::<String>().unwrap(), self.ch);
            }
            '0' => {
                token = Token::new_token(EOF.parse::<String>().unwrap(), 0);
            }
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    token = Token::new_token(self.read_identifier(), self.)
                } else if self.ch.is_ascii_digit() {
                    token = Token::new_token(INT.parse::<String>().unwrap(),
                                             self.read_number().parse::<u8>().unwrap())
                } else {
                    token = Token::new_token(ILLEGAL.parse::<String>().unwrap(), self.ch)
                }
            }
        }

        self.read_char();
        return token;
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        loop {
            if self.ch.is_ascii_alphabetic() {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[position..self.position].to_string()
    }

    pub fn read_number(&mut self) -> String {
        let position = self.position;
        loop {
            if self.ch.is_ascii_digit() {
                self.read_char();
            } else {
                break;
            }
        }
        self.input[position..self.position].to_string()
    }
}

//pub fn is_letter(par:u8) -> bool {
//    match *self {
//        b'A'..=b'Z' | b'a'..=b'z' => true,
//        _ => false
//    }
//
//}