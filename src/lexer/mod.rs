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

fn is_letter(ch: char) -> bool {
    ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z') || ch == '_'
}

fn is_digit(ch: char) -> bool {
    '0' <= ch && ch <= '9'
}

impl Lexer {
    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn next_token(&mut self) -> token::Token {
        self.skip_whitespace();
        let token = match self.ch {
            '=' => token::Token {
                token_type: String::from(token::ASSIGN),
                literal: self.ch.to_string(),
            },
            ';' => token::Token {
                token_type: String::from(token::SEMICOLON),
                literal: self.ch.to_string(),
            },
            '(' => token::Token {
                token_type: String::from(token::LPAREN),
                literal: self.ch.to_string(),
            },
            ')' => token::Token {
                token_type: String::from(token::RPAREN),
                literal: self.ch.to_string(),
            },
            ',' => token::Token {
                token_type: String::from(token::COMMA),
                literal: self.ch.to_string(),
            },
            '+' => token::Token {
                token_type: String::from(token::PLUS),
                literal: self.ch.to_string(),
            },
            '{' => token::Token {
                token_type: String::from(token::LBRACE),
                literal: self.ch.to_string(),
            },
            '}' => token::Token {
                token_type: String::from(token::RBRACE),
                literal: self.ch.to_string(),
            },
            '\0' => token::Token {
                token_type: String::from(token::EOF),
                literal: self.ch.to_string(),
            },
            _ => {
                if is_letter(self.ch) {
                    let literal = self.read_identifier();
                    let token_type = token::get_keywords()
                        .get(&literal)
                        .unwrap_or(&String::from(token::IDENT))
                        .to_string();
                    return token::Token {
                        token_type: token_type,
                        literal,
                    };
                } else if is_digit(self.ch) {
                    let literal = self.read_number();
                    return token::Token {
                        token_type: String::from(token::INT),
                        literal,
                    };
                } else {
                    return token::Token {
                        token_type: String::from(token::ILLEGAL),
                        literal: self.ch.to_string(),
                    };
                }
            }
        };
        // must only be called for the non-default cases.
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

    fn read_identifier(&mut self) -> String {
        let start_pos = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.input[start_pos..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let start_pos = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        self.input[start_pos..self.position].to_string()
    }
}
