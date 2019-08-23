
#[cfg(test)]
mod test {

    use crate::lexer;
    use crate::token;
    use token::Token;

    #[test]
    fn test_next_token() {
        let input = String::from(
            "
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        ",
        );

        let tests = [
            Token::Let,
            Token::Ident {
                literal: String::from("five"),
            },
            Token::Assign,
            Token::Int {
                literal: String::from("5"),
            },
            Token::Semicolon,

            Token::Let,
            Token::Ident {
                literal: String::from("ten"),
            },
            Token::Assign,
            Token::Int {
                literal: String::from("10"),
            },
            Token::Semicolon,

            Token::Let,
            Token::Ident {
                literal: String::from("add"),
            },
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident {
                literal: String::from("x"),
            },
            Token::Comma,
            Token::Ident {
                literal: String::from("y"),
            },
            Token::RParen,
            Token::LBrace,
            Token::Ident {
                literal: String::from("x"),
            },
            Token::Plus,
            Token::Ident {
                literal: String::from("y"),
            },
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident {
                literal: String::from("result"),
            },
            Token::Assign,
            Token::Ident {
                literal: String::from("add"),
            },
            Token::LParen,
            Token::Ident {
                literal: String::from("five"),
            },
            Token::Comma,
            Token::Ident {
                literal: String::from("ten"),
            },
            Token::RParen,
            Token::Semicolon,

            Token::Eof,
        ];
        let mut lexer = lexer::new(&input);
        for test in tests.iter() {
            let tok = lexer.next_token();
            assert_eq!(&tok, test);
        }
    }
}
