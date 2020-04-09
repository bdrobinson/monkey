mod test;

use crate::{
    ast, lexer,
    token::{Token, TokenType},
};

type ParserResult<T> = Result<T, String>;

#[derive(Debug, PartialOrd, PartialEq)]
enum Precedence {
    LOWEST,
    EQUALS,      // ==
    LESSGREATER, // > or <
    SUM,         // +
    PRODUCT,     // *
    PREFIX,      // -X or !X
    CALL,        // myFunction(X)
}

pub struct Parser<'a> {
    lexer: &'a mut lexer::Lexer,
    cur_token: Token,
    peek_token: Token,
}

struct ParsedInfix {
    right: ast::Expression,
    operator: ast::InfixOperator,
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

    // let thing  = one + 6;

    fn parse_expression(&mut self, precedence: Precedence) -> ParserResult<ast::Expression> {
        let mut left_exp = match self.cur_token.token_type() {
            TokenType::Ident => self
                .parse_identifier()
                .map(|i| ast::Expression::Identifier(i)),
            TokenType::Int => self
                .parse_integer_literal()
                .map(|i| ast::Expression::IntegerLiteral(i)),
            TokenType::Bang | TokenType::Minus => self
                .parse_prefix_expression()
                .map(|i| ast::Expression::Prefix(i)),
            TokenType::True | TokenType::False => self
                .parse_boolean_expression()
                .map(|i| ast::Expression::Boolean(i)),
            // TokenType::LParen => self.parse_grouped_expression(),
            _ => Err(format!(
                "Could not find prefix parser for token type {}",
                self.cur_token.token_type().to_string()
            )),
        }?;

        while (self.peek_token.token_type() != TokenType::Semicolon)
            && (precedence < self.peek_precedence())
        {
            self.next_token();
            if let Some(parsed_infix_result) = self.parse_infix_expression() {
                let parsed_infix = parsed_infix_result?;
                left_exp = ast::Expression::Infix(ast::InfixExpression {
                    left: Box::new(left_exp),
                    operator: parsed_infix.operator,
                    right: Box::new(parsed_infix.right),
                });
            } else {
                return Ok(left_exp);
            }
        }
        Ok(left_exp)
    }

    fn parse_expression_statement(&mut self) -> ParserResult<ast::ExpressionStatement> {
        let expression = self.parse_expression(Precedence::LOWEST)?;

        // Semicolons are optional at the end of expression statements to make REPL easier.
        if let Token::Semicolon = self.peek_token {
            self.next_token();
        }
        Ok(ast::ExpressionStatement {
            expression: expression,
        })
    }

    fn parse_prefix_expression(&mut self) -> ParserResult<ast::PrefixExpression> {
        let operator: ast::PrefixOperator = match self.cur_token {
            Token::Bang => Ok(ast::PrefixOperator::Bang),
            Token::Minus => Ok(ast::PrefixOperator::Minus),
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

    fn parse_infix_expression(&mut self) -> Option<ParserResult<ParsedInfix>> {
        let precedence = self.cur_precedence();
        let operator = match self.cur_token {
            Token::Plus => Some(ast::InfixOperator::Plus),
            Token::Minus => Some(ast::InfixOperator::Minus),
            Token::Asterisk => Some(ast::InfixOperator::Multiply),
            Token::Slash => Some(ast::InfixOperator::Divide),
            Token::Gt => Some(ast::InfixOperator::Gt),
            Token::Lt => Some(ast::InfixOperator::Lt),
            Token::Eq => Some(ast::InfixOperator::Eq),
            Token::NotEq => Some(ast::InfixOperator::NotEq),
            _ => None,
        }?;
        self.next_token();
        Some(self.parse_expression(precedence).map(|right| ParsedInfix {
            operator: operator,
            right: right,
        }))
    }

    fn parse_boolean_expression(&mut self) -> ParserResult<ast::BooleanExpression> {
        match self.cur_token {
            Token::True => Ok(ast::BooleanExpression { value: true }),
            Token::False => Ok(ast::BooleanExpression { value: false }),
            _ => Err(String::from("Expected boolean token")),
        }
    }

    // fn parse_grouped_expression(&mut self) -> ParserResult<Option<ast::Expression>> {
    //     self.assert_cur_token_type(TokenType::LParen)?;
    //     self.next_token();

    //     let expression = self.parse_expression(Precedence::LOWEST)?;
    //     if self.peek_token.token_type() != TokenType::RParen {
    //         Ok(None)
    //     } else {
    //         self.next_token();
    //         Ok(Some(expression))
    //     }
    // }

    fn cur_precedence(&self) -> Precedence {
        precedence_for_token_type(&self.cur_token.token_type())
    }

    fn peek_precedence(&self) -> Precedence {
        precedence_for_token_type(&self.peek_token.token_type())
    }
}

fn parser_err<T>(expected_type: TokenType, actual: &Token) -> ParserResult<T> {
    Err(format!(
        "Expected {}, got {:?}",
        expected_type.to_string(),
        actual
    ))
}

fn precedence_for_token_type(token_type: &TokenType) -> Precedence {
    match token_type {
        TokenType::Eq | TokenType::NotEq => Precedence::EQUALS,
        TokenType::Lt | TokenType::Gt => Precedence::LESSGREATER,
        TokenType::Plus | TokenType::Minus => Precedence::SUM,
        TokenType::Slash | TokenType::Asterisk => Precedence::PRODUCT,
        _ => Precedence::LOWEST,
    }
}
