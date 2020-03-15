mod test;

use crate::{
    ast, lexer,
    token::{Token, TokenType},
};

type ParserResult<T> = Result<T, String>;

pub struct Parser<'a> {
    lexer: &'a mut lexer::Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser<'_> {
    pub fn new(lexer: &mut lexer::Lexer) -> Parser {
        let first_token = lexer.next_token();
        let second_token = lexer.next_token();
        Parser {
            lexer: lexer,
            cur_token: first_token,
            peek_token: second_token,
        }
    }

    fn next_token(&mut self) {
        std::mem::swap(&mut self.cur_token, &mut self.peek_token);
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Result<ast::Program, String> {
        let mut program = ast::Program { statements: vec![] };
        loop {
            if let Token::Eof = self.cur_token {
                break;
            };
            let next_statement = self.parse_statement()?;
            program.statements.push(next_statement);
            self.next_token();
        }
        Ok(program)
    }

    fn parse_statement(&mut self) -> ParserResult<ast::Statement> {
        let r: ParserResult<ast::Statement> = match &self.cur_token {
            Token::Let => {
                let let_statement = self.parse_let_statement()?;
                Ok(ast::Statement::Let(let_statement))
            }
            Token::Return => {
                let return_statement = self.parse_return_statement()?;
                Ok(ast::Statement::Return(return_statement))
            }
            a => {
                return Err(format!("Unexpected token: {:?}", a));
            }
        };
        r
    }

    fn parse_identifier(&mut self) -> ParserResult<ast::Identifier> {
        if let Token::Ident { literal } = &self.cur_token {
            Ok(ast::Identifier {
                value: literal.clone(),
            })
        } else {
            parser_err(TokenType::Ident, &self.cur_token)
        }
    }

    fn parse_return_statement(&mut self) -> ParserResult<ast::ReturnStatement> {
        self.assert_cur_token_type(TokenType::Return)?;

        // Next we expect an expression
        self.next_token();
        // let expr = self.parse_expression();

        // end with semi
        self.next_token();
        self.assert_cur_token_type(TokenType::Semicolon)?;

        Ok(ast::ReturnStatement {})
    }

    fn assert_cur_token_type(&self, expected: TokenType) -> Result<(), String> {
        if self.cur_token.token_type() == expected {
            Ok(())
        } else {
            parser_err(expected, &self.cur_token)
        }
    }

    fn parse_let_statement(&mut self) -> ParserResult<ast::LetStatement> {
        // Check we're starting with a Let
        self.assert_cur_token_type(TokenType::Let)?;

        // Identifier should be next
        self.next_token();
        let identifier = self.parse_identifier()?;

        // Then assign
        self.next_token();
        self.assert_cur_token_type(TokenType::Assign)?;

        // Now the expression
        self.next_token();
        let _ = self.parse_expression();

        // Make sure it was terminated
        self.next_token();
        self.assert_cur_token_type(TokenType::Semicolon)?;
        Ok(ast::LetStatement { name: identifier })
    }

    fn parse_expression(&mut self) -> ParserResult<ast::Expression> {
        // just skip for now
        Err(String::from("Not implemented"))
    }
}

fn parser_err<T>(expected_type: TokenType, actual: &Token) -> ParserResult<T> {
    Err(format!(
        "Expected {}, got {:?}",
        expected_type.to_string(),
        actual
    ))
}
