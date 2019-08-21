
#[cfg(test)]
mod test {

    use crate::lexer;
    use crate::token;
    struct LexerTestResult {
        expected_type: String,
        expected_literal: String,
    }

    #[test]
    fn test_next_token() {
        let input = String::from("=+(){},;");
        let tests = [
            LexerTestResult {
                expected_type: String::from(token::ASSIGN),
                expected_literal: String::from("="),
            },
            LexerTestResult {
                expected_type: String::from(token::PLUS),
                expected_literal: String::from("+"),
            },
            LexerTestResult {
                expected_type: String::from(token::LPAREN),
                expected_literal: String::from("("),
            },
            LexerTestResult {
                expected_type: String::from(token::RPAREN),
                expected_literal: String::from(")"),
            },
            LexerTestResult {
                expected_type: String::from(token::LBRACE),
                expected_literal: String::from("{"),
            },
            LexerTestResult {
                expected_type: String::from(token::RBRACE),
                expected_literal: String::from("}"),
            },
            LexerTestResult {
                expected_type: String::from(token::COMMA),
                expected_literal: String::from(","),
            },
            LexerTestResult {
                expected_type: String::from(token::SEMICOLON),
                expected_literal: String::from(";"),
            },
            LexerTestResult {
                expected_type: String::from(token::EOF),
                expected_literal: String::from("\0"),
            },
        ];
        let mut lexer = lexer::new(&input);
        for test in tests.iter() {
            let tok = lexer.next_token();
            assert_eq!(tok.tokenType, test.expected_type);
            assert_eq!(tok.literal, test.expected_literal);
        }
    }
}
