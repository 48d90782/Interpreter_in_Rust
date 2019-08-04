type TokenType<'a> = &'a str;

pub struct Token<'a> {
    pub r#type: TokenType<'a>,
    pub literal: u8,
}

impl Token<'_> {
    pub fn new<'a>() -> Token<'a> {
        Token {
            r#type: "",
            literal: 0,
        }
    }

    pub fn new_token(tok_type: &str, ch: u8) -> Token {
        Token {
            r#type: tok_type,
            literal: ch,
        }
    }
}


