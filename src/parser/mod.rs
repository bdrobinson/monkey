mod test;

use crate::{
    ast, lexer,
    token::{Token, TokenType},
};

type ParserResult<T> = Result<T, String>;

type PrefixParseFn = fn() -> ast::Expression;
type InfixParseFn = fn(ast::Expression) -> ast::Expression;

#[derive(Debug, PartialOrd, PartialEq)]
enum Precedence {
    LOWEST,
    EQUALS,  // == LESSGREATER // > or <
    SUM,     // +
    PRODUCT, // *
    PREFIX,  // -X or !X
    CALL,    // myFunction(X)
}

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

    fn parse_prefix(&mut self, token_type: &TokenType) -> ParserResult<ast::Expression> {
        let expression: ast::Expression = match token_type {
            TokenType::Ident => {
                let ident = self.parse_identifier()?;
                ast::Expression::Identifier(ident)
            }
            TokenType::Int => {
                let int = self.parse_integer_literal()?;
                ast::Expression::IntegerLiteral(int)
            }
            TokenType::Bang | TokenType::Minus => {
                let prefix_expr = self.parse_prefix_expression()?;
                ast::Expression::Prefix(prefix_expr)
            }
            _ => panic!("Could not parse token {}", token_type.to_string()),
        };
        Ok(expression)
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
            _ => {
                let expression = self.parse_expression_statement()?;
                Ok(ast::Statement::Expression(expression))
            }
        };
        r
    }

    fn parse_identifier(&mut self) -> ParserResult<ast::IdentifierExpression> {
        if let Token::Ident { literal } = &self.cur_token {
            Ok(ast::IdentifierExpression {
                value: literal.clone(),
            })
        } else {
            parser_err(TokenType::Ident, &self.cur_token)
        }
    }

    fn parse_integer_literal(&mut self) -> ParserResult<ast::IntegerLiteralExpression> {
        if let Token::Int { literal } = &self.cur_token {
            let parsed = literal.parse::<i64>().map_err(|_| "Could not parse int")?;

            Ok(ast::IntegerLiteralExpression { value: parsed })
        } else {
            parser_err(TokenType::Int, &self.cur_token)
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

    fn assert_peek_token_type(&self, expected: TokenType) -> Result<(), String> {
        if self.peek_token.token_type() == expected {
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
        // skip expresion for now
        // let _ = self.parse_expression();

        // Make sure it was terminated
        self.next_token();
        self.assert_cur_token_type(TokenType::Semicolon)?;
        Ok(ast::LetStatement { name: identifier })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> ParserResult<ast::Expression> {
        let token_type = &self.cur_token.token_type();
        let left_exp = self.parse_prefix(token_type)?;
        Ok(left_exp)
    }

    fn parse_expression_statement(&mut self) -> ParserResult<ast::ExpressionStatement> {
        let expression = self.parse_expression(Precedence::LOWEST)?;
        self.assert_peek_token_type(TokenType::Semicolon)?;
        // Consume the semicolon
        self.next_token();
        Ok(ast::ExpressionStatement {
            expression: expression,
        })
    }

    fn parse_prefix_expression(&mut self) -> ParserResult<ast::PrefixExpression> {
        let operator: ast::PrefixTokenOperator = match self.cur_token {
            Token::Bang => Ok(ast::PrefixTokenOperator::Bang),
            Token::Minus => Ok(ast::PrefixTokenOperator::Minus),
            _ => Err(format!(
                "Expected prefix operator, got {:?}",
                self.cur_token
            )),
        }?;
        self.next_token();
        let right = self.parse_expression(Precedence::PREFIX)?;
        Ok(ast::PrefixExpression {
            operator: operator,
            right: Box::new(right),
        })
    }
}

fn parser_err<T>(expected_type: TokenType, actual: &Token) -> ParserResult<T> {
    Err(format!(
        "Expected {}, got {:?}",
        expected_type.to_string(),
        actual
    ))
}
