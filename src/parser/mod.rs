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
            let parsed = literal.parse::<i64>().map_err(|_| "Could not parse int")?;

            Ok(ast::Expression::IntegerLiteral { value: parsed })
        } else {
            parser_err(TokenType::Int, &self.cur_token)
        }
    }

    fn parse_return_statement(&mut self) -> ParserResult<ast::Statement> {
        self.assert_cur_token_type(TokenType::Return)?;

        // Next we expect an expression
        self.next_token();
        // let expr = self.parse_expression();

        // end with semi
        self.next_token();
        self.assert_cur_token_type(TokenType::Semicolon)?;

        Ok(ast::Statement::Return {})
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
        // let _ = self.parse_expression();

        // Make sure it was terminated
        self.next_token();
        self.assert_cur_token_type(TokenType::Semicolon)?;
        Ok(ast::Statement::Let {
            name: identifier_name,
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
            TokenType::If => self.parse_if_expression(),
            TokenType::Function => self.parse_fn_literal(),
            _ => Err(format!(
                "An expression cannot start with token type {}",
                self.cur_token.token_type()
            )),
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
                left_exp = ast::Expression::Infix {
                    left: Box::new(left_exp),
                    operator: parsed_infix.operator,
                    right: Box::new(parsed_infix.right),
                };
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
            _ => Err(format!(
                "Expected prefix operator, got {:?}",
                self.cur_token
            )),
        }?;
        self.next_token();
        let right = self.parse_expression(Precedence::PREFIX)?;
        Ok(ast::Expression::Prefix {
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

    fn parse_boolean_expression(&mut self) -> ParserResult<ast::Expression> {
        match self.cur_token {
            Token::True => Ok(ast::Expression::Boolean { value: true }),
            Token::False => Ok(ast::Expression::Boolean { value: false }),
            _ => Err(String::from("Expected boolean token")),
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
}

fn parser_err<T>(expected_type: TokenType, actual: &Token) -> ParserResult<T> {
    Err(format!("Expected {}, got {:?}", expected_type, actual))
}

fn precedence_for_token_type(token_type: &TokenType) -> Precedence {
    match token_type {
        TokenType::Eq | TokenType::NotEq => Precedence::EQUALS,
        TokenType::Lt | TokenType::Gt => Precedence::LESSGREATER,
        TokenType::Plus => Precedence::SUM,
        TokenType::Minus => Precedence::MINUS,
        TokenType::Slash => Precedence::DIVIDE,
        TokenType::Asterisk => Precedence::PRODUCT,
        _ => Precedence::LOWEST,
    }
}
