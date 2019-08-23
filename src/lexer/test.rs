
#[cfg(test)]
mod test {

    use crate::lexer;
    use crate::token;
    struct LexerTestResult {
        expected_type: String,
        expected_literal: String,
    }

    fn result(token_type: &str, literal: &str) -> LexerTestResult {
        LexerTestResult {
            expected_type: String::from(token_type),
            expected_literal: String::from(literal),
        }
    }

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
        // let input = String::from("5 ;");
        let tests = [
            result(token::LET, "let"),
            result(token::IDENT, "five"),
            result(token::ASSIGN, "="),
            result(token::INT, "5"),
            result(token::SEMICOLON, ";"),

            result(token::LET, "let"),
            result(token::IDENT, "ten"),
            result(token::ASSIGN, "="),
            result(token::INT, "10"),
            result(token::SEMICOLON, ";"),

            result(token::LET, "let"),
            result(token::IDENT, "add"),
            result(token::ASSIGN, "="),
            result(token::FUNCTION, "fn"),
            result(token::LPAREN, "("),
            result(token::IDENT, "x"),
            result(token::COMMA, ","),
            result(token::IDENT, "y"),
            result(token::RPAREN, ")"),
            result(token::LBRACE, "{"),
            result(token::IDENT, "x"),
            result(token::PLUS, "+"),
            result(token::IDENT, "y"),
            result(token::SEMICOLON, ";"),
            result(token::RBRACE, "}"),
            result(token::SEMICOLON, ";"),
            result(token::LET, "let"),
            result(token::IDENT, "result"),
            result(token::ASSIGN, "="),
            result(token::IDENT, "add"),
            result(token::LPAREN, "("),
            result(token::IDENT, "five"),
            result(token::COMMA, ","),
            result(token::IDENT, "ten"),
            result(token::RPAREN, ")"),
            result(token::SEMICOLON, ";"),

            result(token::EOF, "\0"),
        ];
        let mut lexer = lexer::new(&input);
        for test in tests.iter() {
            let tok = lexer.next_token();
            assert_eq!(tok.token_type, test.expected_type);
            assert_eq!(tok.literal, test.expected_literal);
        }
    }
}
