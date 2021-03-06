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
            right: ast::Expression::IntegerLiteral { value: 5 },
        },
        ast::Statement::Let {
            name: String::from("y"),
            right: ast::Expression::IntegerLiteral { value: 10 },
        },
        ast::Statement::Let {
            name: String::from("foobar"),
            right: ast::Expression::IntegerLiteral { value: 83838383 },
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
        vec![
            ast::Statement::Return {
                value: ast::Expression::IntegerLiteral { value: 1 }
            },
            ast::Statement::Return {
                value: ast::Expression::IntegerLiteral { value: 3 }
            },
        ]
    )
}
fn read_program(input: &'static str) -> ast::Program {
    let mut lexer = lexer::new(input);
    let mut parser = parser::Parser::new(&mut lexer);
    parser.parse_program().unwrap()
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

#[test]
fn test_string_literal_expression() {
    let input = r#"
            "hi";
            "there";
        "#;
    let program = read_program(input);
    assert_eq!(
        program.statements,
        vec![
            ast::Statement::Expression {
                expression: ast::Expression::StringLiteral {
                    value: String::from("hi")
                }
            },
            ast::Statement::Expression {
                expression: ast::Expression::StringLiteral {
                    value: String::from("there")
                }
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
    let first_statement = program_noparens.statements.get(0).unwrap();
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
                consequence: ast::BlockStatement {
                    statements: vec!(ast::Statement::Expression {
                        expression: ast::Expression::Identifier {
                            value: String::from("x")
                        }
                    })
                },
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
                consequence: ast::BlockStatement {
                    statements: vec!(ast::Statement::Expression {
                        expression: ast::Expression::Identifier {
                            value: String::from("x")
                        }
                    })
                },
                alternative: Some(ast::BlockStatement {
                    statements: vec!(ast::Statement::Expression {
                        expression: ast::Expression::Identifier {
                            value: String::from("y")
                        }
                    })
                }),
            }
        })
    )
}

#[test]
fn test_fn_literal() {
    let input = "
        fn (x, y) {
            x + y;
        }
        fn (x) {
            4;
        }
        fn () {
            3;
        }";
    let program = read_program(input);
    assert_eq!(
        program.statements,
        vec!(
            ast::Statement::Expression {
                expression: ast::Expression::FnLiteral {
                    param_names: vec!(String::from("x"), String::from("y")),
                    body: ast::BlockStatement {
                        statements: vec!(ast::Statement::Expression {
                            expression: ast::Expression::Infix {
                                left: Box::new(ast::Expression::Identifier {
                                    value: String::from("x")
                                }),
                                operator: ast::InfixOperator::Plus,
                                right: Box::new(ast::Expression::Identifier {
                                    value: String::from("y")
                                }),
                            }
                        })
                    }
                }
            },
            ast::Statement::Expression {
                expression: ast::Expression::FnLiteral {
                    param_names: vec!(String::from("x")),
                    body: ast::BlockStatement {
                        statements: vec!(ast::Statement::Expression {
                            expression: ast::Expression::IntegerLiteral { value: 4 }
                        })
                    }
                }
            },
            ast::Statement::Expression {
                expression: ast::Expression::FnLiteral {
                    param_names: vec!(),
                    body: ast::BlockStatement {
                        statements: vec!(ast::Statement::Expression {
                            expression: ast::Expression::IntegerLiteral { value: 3 },
                        })
                    }
                }
            }
        )
    )
}

#[test]
fn test_call_expression() {
    let input = "
        add();
        add(1);
        add(1 + 2, 3);
        fn(x, y){}(2);
        ";
    let program = read_program(input);
    assert_eq!(
        program.statements,
        vec!(
            ast::Statement::Expression {
                expression: ast::Expression::CallExpression {
                    left: Box::new(ast::Expression::Identifier {
                        value: String::from("add")
                    }),
                    arguments: vec!()
                }
            },
            ast::Statement::Expression {
                expression: ast::Expression::CallExpression {
                    left: Box::new(ast::Expression::Identifier {
                        value: String::from("add")
                    }),
                    arguments: vec!(ast::Expression::IntegerLiteral { value: 1 })
                }
            },
            ast::Statement::Expression {
                expression: ast::Expression::CallExpression {
                    left: Box::new(ast::Expression::Identifier {
                        value: String::from("add")
                    }),
                    arguments: vec!(
                        ast::Expression::Infix {
                            left: Box::new(ast::Expression::IntegerLiteral { value: 1 }),
                            operator: ast::InfixOperator::Plus,
                            right: Box::new(ast::Expression::IntegerLiteral { value: 2 }),
                        },
                        ast::Expression::IntegerLiteral { value: 3 }
                    )
                }
            },
            ast::Statement::Expression {
                expression: ast::Expression::CallExpression {
                    left: Box::new(ast::Expression::FnLiteral {
                        param_names: vec![String::from("x"), String::from("y")],
                        body: ast::BlockStatement { statements: vec![] },
                    }),
                    arguments: vec!(ast::Expression::IntegerLiteral { value: 2 })
                }
            }
        )
    )
}

#[test]
fn test_block_expression() {
    let input = "
        let a = {
            2;
            { 3 };
        };
        ";
    let program = read_program(input);
    assert_eq!(
        program.statements,
        vec!(ast::Statement::Let {
            name: String::from("a"),
            right: ast::Expression::Block {
                statements: vec![
                    ast::Statement::Expression {
                        expression: ast::Expression::IntegerLiteral { value: 2 }
                    },
                    ast::Statement::Expression {
                        expression: ast::Expression::Block {
                            statements: vec![ast::Statement::Expression {
                                expression: ast::Expression::IntegerLiteral { value: 3 }
                            }]
                        }
                    }
                ]
            }
        })
    )
}
