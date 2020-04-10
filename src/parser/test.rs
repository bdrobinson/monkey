#[cfg(test)]
mod test {
    use crate::{ast, lexer, parser};
    use pretty_assertions::assert_eq;
    #[test]
    fn test_let_statements() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 83838383;
        ";
        let program = read_program(input);
        assert_eq!(program.statements.len(), 3);

        let expected: Vec<ast::Statement> = vec![
            ast::Statement::Let {
                name: String::from("x"),
            },
            ast::Statement::Let {
                name: String::from("y"),
            },
            ast::Statement::Let {
                name: String::from("foobar"),
            },
        ];

        assert_eq!(program.statements, expected);
    }

    #[test]
    fn test_return_statements() {
        let input = "
        return 1;
        return 3;
        ";
        let program = read_program(input);
        assert_eq!(program.statements.len(), 2);
        assert_eq!(
            program.statements,
            vec![ast::Statement::Return {}, ast::Statement::Return {},]
        )
    }
    fn read_program(input: &'static str) -> ast::Program {
        let mut lexer = lexer::new(input);
        let mut parser = parser::Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        program
    }

    #[test]
    fn test_identifier_expression() {
        let input = "
            foobar;
            45;
        ";
        let program = read_program(input);
        assert_eq!(program.statements.len(), 2);
        assert_eq!(
            program.statements,
            vec![
                ast::Statement::Expression {
                    expression: ast::Expression::Identifier {
                        value: String::from("foobar")
                    }
                },
                ast::Statement::Expression {
                    expression: ast::Expression::IntegerLiteral { value: 45 }
                },
            ]
        );
    }

    #[test]
    fn test_prefix_expression() {
        let input = "
            -3;
            !whatever;
        ";
        let program = read_program(input);
        assert_eq!(program.statements.len(), 2);
        assert_eq!(
            program.statements,
            vec![
                ast::Statement::Expression {
                    expression: ast::Expression::Prefix {
                        operator: ast::PrefixOperator::Minus,
                        right: Box::new(ast::Expression::IntegerLiteral { value: 3 }),
                    }
                },
                ast::Statement::Expression {
                    expression: ast::Expression::Prefix {
                        operator: ast::PrefixOperator::Bang,
                        right: Box::new(ast::Expression::Identifier {
                            value: String::from("whatever")
                        }),
                    }
                },
            ],
        )
    }

    #[test]
    fn test_boolean_expression() {
        let input = "
        true;
        false;
        ";
        let program = read_program(input);
        assert_eq!(
            program.statements,
            vec![
                ast::Statement::Expression {
                    expression: ast::Expression::Boolean { value: true }
                },
                ast::Statement::Expression {
                    expression: ast::Expression::Boolean { value: false }
                },
            ]
        )
    }

    fn construct_simple_infix_test_case(
        left: i64,
        op: ast::InfixOperator,
        right: i64,
    ) -> ast::Statement {
        ast::Statement::Expression {
            expression: ast::Expression::Infix {
                left: Box::new(ast::Expression::IntegerLiteral { value: left }),
                operator: op,
                right: Box::new(ast::Expression::IntegerLiteral { value: right }),
            },
        }
    }

    fn run_paren_infix_test(no_parens: &'static str, with_parens: &'static str) {
        let program_noparens = read_program(no_parens);
        let first_statement = program_noparens.statements.iter().nth(0).unwrap();
        let expression = match first_statement {
            ast::Statement::Expression { expression } => expression,
            _ => panic!("Expected expression statement"),
        };
        assert_eq!(expression.to_string(), with_parens);
    }

    #[test]
    fn test_simple_infix_expression() {
        let input = "
            1 + 2;
            2 - 3;
            2 * 3;
            2 / 3;
            2 > 3;
            2 < 3;
            2 == 3;
            2 != 3;
        ";
        let program = read_program(input);
        assert_eq!(program.statements.len(), 8);
        assert_eq!(
            program.statements,
            vec![
                construct_simple_infix_test_case(1, ast::InfixOperator::Plus, 2),
                construct_simple_infix_test_case(2, ast::InfixOperator::Minus, 3),
                construct_simple_infix_test_case(2, ast::InfixOperator::Multiply, 3),
                construct_simple_infix_test_case(2, ast::InfixOperator::Divide, 3),
                construct_simple_infix_test_case(2, ast::InfixOperator::Gt, 3),
                construct_simple_infix_test_case(2, ast::InfixOperator::Lt, 3),
                construct_simple_infix_test_case(2, ast::InfixOperator::Eq, 3),
                construct_simple_infix_test_case(2, ast::InfixOperator::NotEq, 3),
            ],
        )
    }

    #[test]
    fn test_infix_parens() {
        run_paren_infix_test("-a * b", "((-a) * b)");
        run_paren_infix_test("!-a", "(!(-a))");
        run_paren_infix_test("a + b + c", "((a + b) + c)");
        run_paren_infix_test("a - b + c", "(a - (b + c))");
        run_paren_infix_test("a * b * c", "((a * b) * c)");
        run_paren_infix_test("a * b / c", "(a * (b / c))");
        run_paren_infix_test("a + b / c", "(a + (b / c))");
        run_paren_infix_test("(a + b) / c", "((a + b) / c)");
        run_paren_infix_test("a + ((b + c) + d)", "(a + ((b + c) + d))");
    }

    #[test]
    fn test_if_expression() {
        let input = "if (x < y) { x }";
        let program = read_program(input);
        assert_eq!(
            program.statements,
            vec!(ast::Statement::Expression {
                expression: ast::Expression::If {
                    condition: Box::new(ast::Expression::Infix {
                        left: Box::new(ast::Expression::Identifier {
                            value: String::from("x")
                        }),
                        operator: ast::InfixOperator::Lt,
                        right: Box::new(ast::Expression::Identifier {
                            value: String::from("y")
                        })
                    }),
                    consequence: Box::new(ast::BlockStatement {
                        statements: vec!(ast::Statement::Expression {
                            expression: ast::Expression::Identifier {
                                value: String::from("x")
                            }
                        })
                    }),
                    alternative: None,
                }
            })
        )
    }

    #[test]
    fn test_if_else_expression() {
        let input = "if (x < y) { x } else { y }";
        let program = read_program(input);
        assert_eq!(
            program.statements,
            vec!(ast::Statement::Expression {
                expression: ast::Expression::If {
                    condition: Box::new(ast::Expression::Infix {
                        left: Box::new(ast::Expression::Identifier {
                            value: String::from("x")
                        }),
                        operator: ast::InfixOperator::Lt,
                        right: Box::new(ast::Expression::Identifier {
                            value: String::from("y")
                        })
                    }),
                    consequence: Box::new(ast::BlockStatement {
                        statements: vec!(ast::Statement::Expression {
                            expression: ast::Expression::Identifier {
                                value: String::from("x")
                            }
                        })
                    }),
                    alternative: Some(Box::new(ast::BlockStatement {
                        statements: vec!(ast::Statement::Expression {
                            expression: ast::Expression::Identifier {
                                value: String::from("y")
                            }
                        })
                    })),
                }
            })
        )
    }
}
