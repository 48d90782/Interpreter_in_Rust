use lazy_static::lazy_static;
use std::collections::hash_map::HashMap;
use std::sync::Mutex;
use crate::constants::{LET, FUNCTION, IDENT};

type TokenType = String;

lazy_static! {
static ref HASHMAP: Mutex<HashMap<String, TokenType>> = {
    let mut m = HashMap::new();
    m.insert("let".to_string(), LET.to_string());
    m.insert("fn".to_string(), FUNCTION.to_string());
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

    pub fn new_token(tok_type: TokenType, literal: char) -> Token {
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
        match HASHMAP.lock().unwrap().get(&ident) {
            Some(val) => {
                val.clone()
            }
            _ => {
                IDENT.to_string()
            }
        }
    }

//    fn eq(&self, other: &U) -> bool {
//
//    }
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


