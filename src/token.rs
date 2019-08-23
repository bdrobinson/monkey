pub fn token_from_word(literal: &String) -> Option<Token> {
    match literal.as_str() {
        "fn" => Some(Token::Function),
        "let" => Some(Token::Let),
        _ => None,
    }
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Illegal { literal: String },
    Eof,
    Ident { literal: String },
    Int { literal: String },
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
}
