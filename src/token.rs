use lazy_static::lazy_static;
use std::collections::hash_map::HashMap;
use std::sync::Mutex;

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// identifiers and literals
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

// operators
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";
pub const LT: &str = "<";
pub const GT: &str = ">";
pub const EQ: &str = "==";
pub const NOT_EQ: &str = "!=";

// delimiters
pub const COMMA: &str = "COMMA";
pub const SEMICOLON: &str = "SEMICOLON";

pub const LPAREN: &str = "LPAREN";
pub const RPAREN: &str = "RPAREN";
pub const LBRACE: &str = "LBRACE";
pub const RBRACE: &str = "RBRACE";

// keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const RETURN: &str = "RETURN";

// TODO maybe not String but &str ???
type TokenType = String;

lazy_static! {
static ref HASHMAP: Mutex<HashMap<String, TokenType>> = {
    let mut m = HashMap::new();
    m.insert("let".to_string(), LET.to_string());
    m.insert("fn".to_string(), FUNCTION.to_string());
    m.insert("true".to_string(), TRUE.to_string());
    m.insert("false".to_string(), FALSE.to_string());
    m.insert("if".to_string(), IF.to_string());
    m.insert("else".to_string(), ELSE.to_string());
    m.insert("return".to_string(), RETURN.to_string());
    Mutex::new(m)
    };
}

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
            Some(val) => {
                val.clone()
            }
            _ => {
                IDENT.to_string()
            }
        }
    }

}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        if (self.r#type == other.r#type)
            && (self.literal == other.literal) {
            return true;
        }
        false
    }
}


