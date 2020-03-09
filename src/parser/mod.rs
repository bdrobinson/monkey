mod test;

use crate::{ast, lexer, token::Token};

type ParserResult<T> = Result<T, String>;

struct Parser<'a> {
    lexer: &'a mut lexer::Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser<'_> {
    fn new(lexer: &mut lexer::Lexer) -> Parser {
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

    fn parse_program(&mut self) -> Result<ast::Program, String> {
        let mut program = ast::Program { statements: vec![] };
        loop {
            let next_statement: Option<ast::Statement> = match &self.cur_token {
                Token::Let => {
                    let let_statement = self.parse_let_statement()?;
                    Some(ast::Statement::Let(let_statement))
                }
                Token::Eof => None,
                a => {
                    return Err(format!("Unexpected token: {:?}", a));
                }
            };
            match next_statement {
                Some(next_statement) => {
                    program.statements.push(next_statement);
                }
                None => break,
            };
            self.next_token();
        }
        Ok(program)
    }

    fn parse_let_statement(&mut self) -> ParserResult<ast::LetStatement> {
        self.next_token();
        let identifier = match &self.cur_token {
            Token::Ident { literal } => ast::Identifier {
                value: literal.clone(),
            },
            a => {
                return Err(parserErr("Ident", &a));
            }
        };
        self.next_token();

        match &self.cur_token {
            Token::Assign => {}
            a => {
                return Err(parserErr("Assign", &a));
            }
        }

        self.next_token();
        let _ = self.parse_expression();

        self.next_token();
        match &self.cur_token {
            Token::Semicolon => {}
            a => {
                return Err(parserErr("Semi", &a));
            }
        }

        Ok(ast::LetStatement { name: identifier })
    }

    fn parse_expression(&mut self) -> ParserResult<ast::Expression> {
        // just skip for now
        Err(String::from("Not implemented"))
    }
}

fn parserErr(expected: &'static str, actual: &Token) -> String {
    format!("Expected {}, got {:?}", expected, actual)
}
