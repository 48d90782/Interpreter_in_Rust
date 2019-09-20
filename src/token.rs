type TokenType = String;

pub struct Token {
    pub r#type: TokenType,
    pub literal: u8,
}

impl Token {
    pub fn new() -> Token {
        Token {
            r#type: "".to_string(),
            literal: 0,
        }
    }

    pub fn new_token(tok_type: String, ch: u8) -> Token {
        Token {
            r#type: tok_type,
            literal: ch,
        }
    }
}


