use std::fmt;

pub fn token_from_word(literal: &str) -> Option<Token> {
    match literal {
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

#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    Illegal { literal: String },
    Eof,
    Ident { literal: String },
    Int { literal: String },
    String { literal: String },
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

impl Token {
    pub fn token_type(&self) -> TokenType {
        match self {
            Token::Illegal { .. } => TokenType::Illegal,
            Token::Eof => TokenType::Eof,
            Token::Ident { .. } => TokenType::Ident,
            Token::Int { .. } => TokenType::Int,
            Token::String { .. } => TokenType::String,
            Token::Assign => TokenType::Assign,
            Token::Plus => TokenType::Plus,
            Token::Minus => TokenType::Minus,
            Token::Comma => TokenType::Comma,
            Token::Semicolon => TokenType::Semicolon,
            Token::LParen => TokenType::LParen,
            Token::RParen => TokenType::RParen,
            Token::LBrace => TokenType::LBrace,
            Token::RBrace => TokenType::RBrace,
            Token::Function => TokenType::Function,
            Token::Let => TokenType::Let,
            Token::Lt => TokenType::Lt,
            Token::Gt => TokenType::Gt,
            Token::Bang => TokenType::Bang,
            Token::Asterisk => TokenType::Asterisk,
            Token::Slash => TokenType::Slash,
            Token::If => TokenType::If,
            Token::Else => TokenType::Else,
            Token::Return => TokenType::Return,
            Token::True => TokenType::True,
            Token::False => TokenType::False,
            Token::Eq => TokenType::Eq,
            Token::NotEq => TokenType::NotEq,
        }
    }
}

#[derive(PartialEq, Hash, Eq, Debug)]
pub enum TokenType {
    Illegal,
    Eof,
    Ident,
    Int,
    String,
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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            TokenType::Illegal => "Illegal",
            TokenType::Eof => "Eof",
            TokenType::Ident => "Ident",
            TokenType::Int => "Int",
            TokenType::String => "String",
            TokenType::Assign => "Assign",
            TokenType::Plus => "Plus",
            TokenType::Minus => "Minus",
            TokenType::Comma => "Comma",
            TokenType::Semicolon => "Semicolon",
            TokenType::LParen => "LParen",
            TokenType::RParen => "RParen",
            TokenType::LBrace => "LBrace",
            TokenType::RBrace => "RBrace",
            TokenType::Function => "Function",
            TokenType::Let => "Let",
            TokenType::Lt => "Lt",
            TokenType::Gt => "Gt",
            TokenType::Bang => "Bang",
            TokenType::Asterisk => "Asterisk",
            TokenType::Slash => "Slash",
            TokenType::If => "If",
            TokenType::Else => "Else",
            TokenType::Return => "Return",
            TokenType::True => "True",
            TokenType::False => "False",
            TokenType::Eq => "Eq",
            TokenType::NotEq => "NotEq",
        };
        write!(f, "{}", string)
    }
}
