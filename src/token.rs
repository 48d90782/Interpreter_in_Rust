use std::collections::hash_map::HashMap;

type TokenType = String;

pub struct Token {
    pub r#type: TokenType,
    pub literal: String,
    pub keywords: HashMap<String, TokenType>,
}

impl Token {
    pub fn new() -> Token {
        Token {
            r#type: "".to_string(),
            literal: "".to_string(),
        }
    }

    pub fn new_token(tok_type: TokenType, literal: u8) -> Token {
        Token {
            r#type: tok_type,
            literal: literal.to_string(),
        }
    }

    pub fn new_token_string(tok_type: TokenType, literal: String) -> Token {
        Token {
            r#type: tok_type,
            literal,
        }
    }

    pub fn lookup_ident(ident: String) -> TokenType {
        "LET".to_string()
    }
}


