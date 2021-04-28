#[derive(Debug, Copy, Clone)]
pub enum Token {
    ILLEGAL,
    EOF,

    // identifiers and literals
    IDENT(String),
    INT,

    // operators
    ASSIGN,
    PLUG,
    MINUS,
    BANG,
    ASTERISK,
    SLASH,
    LT,
    GT,
    EQ,
    NOT_EQ,

    // delimiters
    COMMA,
    SEMICOLON,

    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,

    // keywords
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
}


// delimiters
#[derive(Debug, Copy, Clone)]
pub enum Delimiters {}

// TODO maybe not String but &str ???
type TokenType = String;

// lazy_static! {
//     static ref HASHMAP: Mutex<HashMap<String, TokenType>> = {
//         let mut m = HashMap::new();
//         m.insert("let".to_string(), LET);
//         m.insert("fn".to_string(), FUNCTION);
//         m.insert("true".to_string(), TRUE);
//         m.insert("false".to_string(), FALSE);
//         m.insert("if".to_string(), IF);
//         m.insert("else".to_string(), ELSE);
//         m.insert("return".to_string(), RETURN);
//         Mutex::new(m)
//     };
// }

#[derive(Debug, Clone)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new() -> Token {
        Token {
            r#type: "".to_string(),
            literal: "".to_string(),
        }
    }

    pub fn new_token(tok_type: TokenType, literal: String) -> Token {
        Token {
            r#type: tok_type,
            literal,
        }
    }

    pub fn lookup_ident(ident: String) -> TokenType {
        match HASHMAP.lock().unwrap().get(&ident) {
            Some(val) => val.clone(),
            _ => IDENT.to_string(),
        }
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        if (self.r#type == other.r#type) && (self.literal == other.literal) {
            return true;
        }
        false
    }
}
