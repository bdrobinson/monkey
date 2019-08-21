mod test;

use crate::token;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

pub fn new(input: &str) -> Lexer {
    let mut l = Lexer {
        input: String::from(input),
        position: 0,
        read_position: 0,
        ch: '\0',
    };
    l.read_char();
    l
}

impl Lexer {
    fn next_token(&mut self) -> token::Token {
        let token = match self.ch {
            '=' => token::Token {
                tokenType: String::from(token::ASSIGN),
                literal: self.ch.to_string(),
            },
            ';' => token::Token {
                tokenType: String::from(token::SEMICOLON),
                literal: self.ch.to_string(),
            },
            '(' => token::Token {
                tokenType: String::from(token::LPAREN),
                literal: self.ch.to_string(),
            },
            ')' => token::Token {
                tokenType: String::from(token::RPAREN),
                literal: self.ch.to_string(),
            },
            ',' => token::Token {
                tokenType: String::from(token::COMMA),
                literal: self.ch.to_string(),
            },
            '+' => token::Token {
                tokenType: String::from(token::PLUS),
                literal: self.ch.to_string(),
            },
            '{' => token::Token {
                tokenType: String::from(token::LBRACE),
                literal: self.ch.to_string(),
            },
            '}' => token::Token {
                tokenType: String::from(token::RBRACE),
                literal: self.ch.to_string(),
            },
            '\0' => token::Token {
                tokenType: String::from(token::EOF),
                literal: self.ch.to_string(),
            },
            _ => {
                panic!("Unexpected token");
            }
        };
        self.read_char();
        token
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            let all_chars: Vec<char> = self.input.chars().collect();
            self.ch = all_chars[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
}
