pub fn token_from_word(literal: &String) -> Option<Token> {
    match literal.as_str() {
        "fn" => Some(Token::Function),
        "let" => Some(Token::Let),
        "if" => Some(Token::If),
        "else" => Some(Token::Else),
        "return" => Some(Token::Return),
        "true" => Some(Token::True),
        "false" => Some(Token::False),
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
    Minus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
    Lt,
    Gt,
    Bang,
    Asterisk,
    Slash,
    If,
    Else,
    Return,
    True,
    False,
    Eq,
    NotEq,
}
