type TokenType = str;

const ILLEGAL: &str = "ILLEGAL";
const EOF: &str = "EOF";

const IDENT: &str = "IDENT";
const INT: &str = "INT";

const ASSIGN: &str = "ASSIGN";
const PLUS: &str = "PLUS";

const COMMA: &str = "COMMA";
const SEMICOLON: &str = "SEMICOLON";

const LPAREN: &str = "LPAREN";
const RPAREN: &str = "RPAREN";
const LBRACE: &str = "LBRACE";
const RBRACE: &str = "RBRACE";

const FUNCTION: &str = "FUNCTION";
const LET: &str = "LET";


struct Token {
    r#type: TokenType,
    literal: String,
}

