
use std::collections::HashMap;

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

// Identifiers + literals
pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

// Operators
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";

// Delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";

// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";

pub fn get_keywords() -> HashMap<String, String> {
    let mut keywords: HashMap<String, String> = HashMap::new();
    keywords.insert(String::from("fn"), String::from(FUNCTION));
    keywords.insert(String::from("let"), String::from(LET));
    keywords
}

pub struct Token {
    pub tokenType: String,
    pub literal: String,
}
