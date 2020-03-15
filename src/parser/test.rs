#[cfg(test)]
mod test {
    use crate::{ast, lexer, parser};
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
            ast::Statement::Let(ast::LetStatement {
                name: ast::Identifier {
                    value: String::from("x"),
                },
            }),
            ast::Statement::Let(ast::LetStatement {
                name: ast::Identifier {
                    value: String::from("y"),
                },
            }),
            ast::Statement::Let(ast::LetStatement {
                name: ast::Identifier {
                    value: String::from("foobar"),
                },
            }),
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
    }
    fn read_program(input: &'static str) -> ast::Program {
        let mut lexer = lexer::new(input);
        let mut parser = parser::Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        program
    }
}
