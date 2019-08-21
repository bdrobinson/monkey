
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
        let input = String::from("=+(){},;");
        let tests = [
            result(token::ASSIGN, "="),
            result(token::PLUS, "+"),
            result(token::LPAREN, "("),
            result(token::RPAREN, ")"),
            result(token::LBRACE, "{"),
            result(token::RBRACE, "}"),
            result(token::COMMA, ","),
            result(token::SEMICOLON, ";"),
            result(token::EOF, "\0"),
        ];
        let mut lexer = lexer::new(&input);
        for test in tests.iter() {
            let tok = lexer.next_token();
            assert_eq!(tok.tokenType, test.expected_type);
            assert_eq!(tok.literal, test.expected_literal);
        }
    }
}
