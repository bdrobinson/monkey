mod test;

use crate::{
    ast, lexer,
    token::{Token, TokenType},
};

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken { expected: TokenType, actual: Token },
    InvalidExpression { first_token: Token },
}

type ParserResult<T> = Result<T, ParserError>;

#[derive(Debug, PartialOrd, PartialEq)]
enum Precedence {
    LOWEST,
    EQUALS,      // ==
    LESSGREATER, // > or <
    MINUS,       // -
    SUM,         // +
    PRODUCT,     // *
    DIVIDE,      // /
    PREFIX,      // -X or !X
    CALL,        // myFunction(X)
}

pub struct Parser<'a> {
    lexer: &'a mut lexer::Lexer,
    cur_token: Token,
    peek_token: Token,
}

enum ParsedInfix {
    Infix {
        right: ast::Expression,
        operator: ast::InfixOperator,
    },
    Call {
        args: Vec<ast::Expression>,
    },
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

    pub fn parse_program(&mut self) -> Result<ast::Program, ParserError> {
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
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        };
        r
    }

    fn parse_identifier(&mut self) -> ParserResult<String> {
        if let Token::Ident { literal } = &self.cur_token {
            Ok(literal.clone())
        } else {
            parser_err(TokenType::Ident, &self.cur_token)
        }
    }

    fn parse_integer_literal(&mut self) -> ParserResult<ast::Expression> {
        if let Token::Int { literal } = &self.cur_token {
            // It's impossible for this to go wrong as we've already
            // established it's an integer.
            let parsed = literal.parse::<i64>().unwrap();

            Ok(ast::Expression::IntegerLiteral { value: parsed })
        } else {
            parser_err(TokenType::Int, &self.cur_token)
        }
    }

    fn parse_return_statement(&mut self) -> ParserResult<ast::Statement> {
        self.assert_cur_token_type(TokenType::Return)?;

        // Next we expect an expression
        self.next_token();
        let expr = self.parse_expression(Precedence::LOWEST)?;

        // end with semi
        self.next_token();
        self.assert_cur_token_type(TokenType::Semicolon)?;

        Ok(ast::Statement::Return { value: expr })
    }

    fn assert_cur_token_type(&self, expected: TokenType) -> Result<(), ParserError> {
        if self.cur_token.token_type() == expected {
            Ok(())
        } else {
            parser_err(expected, &self.cur_token)
        }
    }

    fn parse_let_statement(&mut self) -> ParserResult<ast::Statement> {
        // Check we're starting with a Let
        self.assert_cur_token_type(TokenType::Let)?;

        // Identifier should be next
        self.next_token();
        let identifier_name = self.parse_identifier()?;

        // Then assign
        self.next_token();
        self.assert_cur_token_type(TokenType::Assign)?;

        // Now the expression
        self.next_token();
        // skip expresion for now
        let expr = self.parse_expression(Precedence::LOWEST)?;

        // Make sure it was terminated
        self.next_token();
        self.assert_cur_token_type(TokenType::Semicolon)?;
        Ok(ast::Statement::Let {
            name: identifier_name,
            right: expr,
        })
    }

    fn parse_fn_literal(&mut self) -> ParserResult<ast::Expression> {
        self.assert_cur_token_type(TokenType::Function)?;
        self.next_token();

        self.assert_cur_token_type(TokenType::LParen)?;
        self.next_token();

        let mut param_names: Vec<String> = vec![];
        while self.cur_token.token_type() != TokenType::RParen {
            let name = self.parse_identifier()?;
            param_names.push(name);
            self.next_token();

            if self.cur_token.token_type() == TokenType::Comma {
                self.next_token();
            }
        }

        // Current token is now RParen
        self.next_token();

        let body = self.parse_block_statement()?;

        Ok(ast::Expression::FnLiteral {
            param_names: param_names,
            body: body,
        })
    }

    fn parse_expression(&mut self, precedence: Precedence) -> ParserResult<ast::Expression> {
        let mut left_exp: ast::Expression = match self.cur_token.token_type() {
            TokenType::Ident => self
                .parse_identifier()
                .map(|s| ast::Expression::Identifier { value: s }),
            TokenType::Int => self.parse_integer_literal(),
            TokenType::Bang | TokenType::Minus => self.parse_prefix_expression(),
            TokenType::True | TokenType::False => self.parse_boolean_expression(),
            TokenType::LParen => self.parse_grouped_expression(),
            TokenType::LBrace => self
                .parse_block_statement()
                .map(|block| ast::Expression::Block {
                    statements: block.statements,
                }),
            TokenType::If => self.parse_if_expression(),
            TokenType::Function => self.parse_fn_literal(),
            TokenType::String => self.parse_string_literal(),
            _ => Err(ParserError::InvalidExpression {
                first_token: self.cur_token.clone(),
            }),
        }?;

        // This algorithm can essentially iterate horizontally using
        // while, or vertically using recursion.
        // Horizontal: (((1 + 2) + 3) + 4)
        // Vertical:   (1 + (2 + (3 + 4)))
        while (self.peek_token.token_type() != TokenType::Semicolon)
            && (precedence < self.peek_precedence())
        {
            self.next_token();
            // cur token is a potential infix operator

            if let Some(parsed_infix_result) = self.parse_infix_expression() {
                let parsed_infix = parsed_infix_result?;
                match parsed_infix {
                    ParsedInfix::Infix { operator, right } => {
                        left_exp = ast::Expression::Infix {
                            left: Box::new(left_exp),
                            operator: operator,
                            right: Box::new(right),
                        }
                    }
                    ParsedInfix::Call { args } => {
                        left_exp = ast::Expression::CallExpression {
                            left: Box::new(left_exp),
                            arguments: args,
                        }
                    }
                }
            } else {
                // it wasn't an infix op – expression is done.
                return Ok(left_exp);
            }
        }
        Ok(left_exp)
    }

    fn parse_if_expression(&mut self) -> ParserResult<ast::Expression> {
        self.assert_cur_token_type(TokenType::If)?;
        self.next_token();

        self.assert_cur_token_type(TokenType::LParen)?;
        self.next_token();

        let condition = self.parse_expression(Precedence::LOWEST)?;
        self.next_token();

        self.assert_cur_token_type(TokenType::RParen)?;
        self.next_token();

        let consequence = self.parse_block_statement()?;

        let alternative = if Token::Else == self.peek_token {
            self.next_token(); // now we're on the else
            self.next_token(); // now we're on the start of the block statement
            Some(self.parse_block_statement()?)
        } else {
            None
        };
        Ok(ast::Expression::If {
            condition: Box::new(condition),
            consequence: consequence,
            alternative: alternative,
        })
    }

    fn parse_block_statement(&mut self) -> ParserResult<ast::BlockStatement> {
        self.assert_cur_token_type(TokenType::LBrace)?;
        self.next_token();
        // cur token is now either an RBrace or the start of an expression statement.

        let mut statements: Vec<ast::Statement> = vec![];
        while self.cur_token != Token::RBrace {
            let statement = self.parse_statement()?;
            statements.push(statement);
            self.next_token();
        }
        Ok(ast::BlockStatement {
            statements: statements,
        })
    }

    fn parse_expression_statement(&mut self) -> ParserResult<ast::Statement> {
        let expression = self.parse_expression(Precedence::LOWEST)?;

        // Semicolons are optional at the end of expression statements to make REPL easier.
        if let Token::Semicolon = self.peek_token {
            self.next_token();
        }
        Ok(ast::Statement::Expression {
            expression: expression,
        })
    }

    fn parse_prefix_expression(&mut self) -> ParserResult<ast::Expression> {
        let operator: ast::PrefixOperator = match self.cur_token {
            Token::Bang => Ok(ast::PrefixOperator::Bang),
            Token::Minus => Ok(ast::PrefixOperator::Minus),
            _ => unreachable!(),
        }?;
        self.next_token();
        let right = self.parse_expression(Precedence::PREFIX)?;
        Ok(ast::Expression::Prefix {
            operator: operator,
            right: Box::new(right),
        })
    }

    fn parse_call_args(&mut self) -> ParserResult<Vec<ast::Expression>> {
        self.assert_cur_token_type(TokenType::LParen)?;
        self.next_token();
        let mut args: Vec<ast::Expression> = vec![];
        while self.cur_token != Token::RParen {
            let expr = self.parse_expression(Precedence::LOWEST)?;
            args.push(expr);
            self.next_token();
            if self.cur_token == Token::Comma {
                self.next_token();
            }
        }
        Ok(args)
    }

    fn parse_infix_expression(&mut self) -> Option<ParserResult<ParsedInfix>> {
        let precedence = self.cur_precedence();
        let operator_token = &self.cur_token;
        if operator_token == &Token::LParen {
            // it's a call expression!
            Some(
                self.parse_call_args()
                    .map(|args| ParsedInfix::Call { args: args }),
            )
        } else {
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
            Some(
                self.parse_expression(precedence)
                    .map(|right| ParsedInfix::Infix {
                        operator: operator,
                        right: right,
                    }),
            )
        }
    }

    fn parse_boolean_expression(&mut self) -> ParserResult<ast::Expression> {
        match self.cur_token {
            Token::True => Ok(ast::Expression::Boolean { value: true }),
            Token::False => Ok(ast::Expression::Boolean { value: false }),
            _ => unreachable!(),
        }
    }

    fn parse_grouped_expression(&mut self) -> ParserResult<ast::Expression> {
        self.assert_cur_token_type(TokenType::LParen)?;
        self.next_token();

        let expression = self.parse_expression(Precedence::LOWEST)?;
        self.next_token();
        self.assert_cur_token_type(TokenType::RParen)?;
        Ok(expression)
    }

    fn cur_precedence(&self) -> Precedence {
        precedence_for_token_type(&self.cur_token.token_type())
    }

    fn peek_precedence(&self) -> Precedence {
        precedence_for_token_type(&self.peek_token.token_type())
    }

    fn parse_string_literal(&mut self) -> ParserResult<ast::Expression> {
        if let Token::String { literal } = &self.cur_token {
            Ok(ast::Expression::StringLiteral {
                value: literal.clone(),
            })
        } else {
            unreachable!()
        }
    }
}

fn parser_err<T>(expected_type: TokenType, actual: &Token) -> ParserResult<T> {
    Err(ParserError::UnexpectedToken {
        expected: expected_type,
        actual: actual.clone(),
    })
}

fn precedence_for_token_type(token_type: &TokenType) -> Precedence {
    match token_type {
        TokenType::Eq | TokenType::NotEq => Precedence::EQUALS,
        TokenType::Lt | TokenType::Gt => Precedence::LESSGREATER,
        TokenType::Plus => Precedence::SUM,
        TokenType::Minus => Precedence::MINUS,
        TokenType::Slash => Precedence::DIVIDE,
        TokenType::Asterisk => Precedence::PRODUCT,
        TokenType::LParen => Precedence::CALL,
        _ => Precedence::LOWEST,
    }
}
