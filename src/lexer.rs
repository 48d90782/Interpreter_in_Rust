use crate::token::Token;
use crate::constants::{ASSIGN, SEMICOLON, LPAREN, RPAREN, COMMA, PLUS, LBRACE, RBRACE, EOF, INT, ILLEGAL};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char, //TODO think about using Vec<u8> as byte. So, in that case we need no function new_token_string
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '0',
        };
        lexer.read_char();
        lexer
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            let (_, chr) = self.input.char_indices().nth(self.read_position).unwrap();
            self.ch = chr;
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let mut token: Token = Token::new();

        self.skip_whitespace();

        match self.ch {
            '=' => {
                token = Token::new_token(
                    ASSIGN.parse::<String>().unwrap(),
                    self.ch.to_string()
                );
            }
            ';' => {
                token = Token::new_token(
                    SEMICOLON.parse::<String>().unwrap(),
                    self.ch.to_string()
                );
            }
            '(' => {
                token = Token::new_token(
                    LPAREN.parse::<String>().unwrap(),
                    self.ch.to_string()
                );
            }
            ')' => {
                token = Token::new_token(
                    RPAREN.parse::<String>().unwrap(),
                    self.ch.to_string()
                );
            }
            ',' => {
                token = Token::new_token(
                    COMMA.parse::<String>().unwrap(),
                    self.ch.to_string()
                );
            }
            '+' => {
                token = Token::new_token(
                    PLUS.parse::<String>().unwrap(),
                    self.ch.to_string()
                );
            }
            '{' => {
                token = Token::new_token(
                    LBRACE.parse::<String>().unwrap(),
                    self.ch.to_string()
                );
            }
            '}' => {
                token = Token::new_token(
                    RBRACE.parse::<String>().unwrap(),
                    self.ch.to_string()
                );
            }
            // null char meaning EOF
            '\0' => {
                token = Token::new_token(
                    EOF.parse::<String>().unwrap(),
                    '\0'.to_string()
                );
            }
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    // if this is not a one of the predefined tokens
                    // read all from whitespace to whitespace _ some_input _
                    // then we check if this is keyword
                    // if not - this is IDENT
                    let literal = self.read_identifier();
                    let tok_type = Token::lookup_ident(literal.clone());
                    token = Token::new_token(tok_type.to_string().clone(), literal.clone());
                    return token;
                } else if self.ch.is_ascii_digit() {
                    // TODO think about float, decimals and other (i32,i64)
                    // Now only INT type supported
                    let tok_type = INT.parse::<String>().unwrap();
                    let literal = self.read_number();
                    token = Token::new_token(tok_type.clone(), literal.clone());
                    return token;
                } else {
                    // the last chance --> ILLEGAL input
                    token = Token::new_token(ILLEGAL.parse::<String>().unwrap(), self.ch.to_string())
                }
            }
        }

        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> String {
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

    fn read_number(&mut self) -> String {
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

    fn skip_whitespace(&mut self) {
        // skip the whitespace
        // tabs
        // new line
        // return
        match self.ch as char {
            ' ' => {
                self.read_char();
            }
            '\t' => {
                self.read_char();
            }
            '\n' => {
                self.read_char();
            }
            '\r' => {
                self.read_char();
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod lexer_test {
    use crate::lexer::Lexer;
    use crate::token::Token;
    use crate::constants::{LET, IDENT, ASSIGN, INT, SEMICOLON, FUNCTION, LPAREN, COMMA, RPAREN, LBRACE, PLUS, RBRACE, EOF};


    #[test]
    fn test_next_token() {
        let input: String =
            "let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);".to_string();

        let mut l = Lexer::new(input);

        let tokens: [Token; 37] = [
            Token::new_token(LET.to_string(), "let".to_string()),
            Token::new_token(IDENT.to_string(), "five".to_string()),
            Token::new_token(ASSIGN.to_string(), "=".to_string()),
            Token::new_token(INT.to_string(), "5".to_string()),
            Token::new_token(SEMICOLON.to_string(), ";".to_string()),
            Token::new_token(LET.to_string(), "let".to_string()),
            Token::new_token(IDENT.to_string(), "ten".to_string()),
            Token::new_token(ASSIGN.to_string(), "=".to_string()),
            Token::new_token(INT.to_string(), "10".to_string()),
            Token::new_token(SEMICOLON.to_string(), ";".to_string()),
            Token::new_token(LET.to_string(), "let".to_string()),
            Token::new_token(IDENT.to_string(), "add".to_string()),
            Token::new_token(ASSIGN.to_string(), "=".to_string()),
            Token::new_token(FUNCTION.to_string(), "fn".to_string()),
            Token::new_token(LPAREN.to_string(), "(".to_string()),
            Token::new_token(IDENT.to_string(), "x".to_string()),
            Token::new_token(COMMA.to_string(), ",".to_string()),
            Token::new_token(IDENT.to_string(), "y".to_string()),
            Token::new_token(RPAREN.to_string(), ")".to_string()),
            Token::new_token(LBRACE.to_string(), "{".to_string()),
            Token::new_token(IDENT.to_string(), "x".to_string()),
            Token::new_token(PLUS.to_string(), "+".to_string()),
            Token::new_token(IDENT.to_string(), "y".to_string()),
            Token::new_token(SEMICOLON.to_string(), ";".to_string()),
            Token::new_token(RBRACE.to_string(), "}".to_string()),
            Token::new_token(SEMICOLON.to_string(), ";".to_string()),
            Token::new_token(LET.to_string(), "let".to_string()),
            Token::new_token(IDENT.to_string(), "result".to_string()),
            Token::new_token(ASSIGN.to_string(), "=".to_string()),
            Token::new_token(IDENT.to_string(), "add".to_string()),
            Token::new_token(LPAREN.to_string(), "(".to_string()),
            Token::new_token(IDENT.to_string(), "five".to_string()),
            Token::new_token(COMMA.to_string(), ",".to_string()),
            Token::new_token(IDENT.to_string(), "ten".to_string()),
            Token::new_token(RPAREN.to_string(), ")".to_string()),
            Token::new_token(SEMICOLON.to_string(), ";".to_string()),
            Token::new_token(EOF.to_string(), "\0".to_string()),
        ];

        for t in tokens.iter() {
            // implemented partialeq for struct
            assert_eq!(*t, l.next_token());
        }
    }
}