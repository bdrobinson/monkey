#[cfg(test)]
mod test {

    use crate::ast;
    use crate::errors;
    use crate::eval;
    use crate::lexer;
    use crate::object::{environment::Environment, Object};
    use crate::parser;
    use core::cell::RefCell;
    use std::rc::Rc;

    struct TestEvalIntCase {
        input: &'static str,
        output: i64,
    }
    #[test]
    fn test_eval_integer_expression() {
        let tests: Vec<TestEvalIntCase> = vec![
            TestEvalIntCase {
                input: "5",
                output: 5,
            },
            TestEvalIntCase {
                input: "10",
                output: 10,
            },
            TestEvalIntCase {
                input: "10 + 15",
                output: 25,
            },
            TestEvalIntCase {
                input: "2 - 12",
                output: -10,
            },
            TestEvalIntCase {
                input: "5 + 10 / 2 ",
                output: 10,
            },
            TestEvalIntCase {
                input: "-5",
                output: -5,
            },
        ];
        for test in tests {
            assert_eq!(
                Object::Integer(test.output),
                *eval_expression_statement(test.input)
            );
        }
    }

    struct TestEvalBoolCase {
        input: &'static str,
        output: bool,
    }
    #[test]
    fn test_eval_boolean_expression() {
        let tests: Vec<TestEvalBoolCase> = vec![
            TestEvalBoolCase {
                input: "true",
                output: true,
            },
            TestEvalBoolCase {
                input: "false",
                output: false,
            },
            TestEvalBoolCase {
                input: "2 < 3",
                output: true,
            },
            TestEvalBoolCase {
                input: "2 < 1",
                output: false,
            },
            TestEvalBoolCase {
                input: "2 > 3",
                output: false,
            },
            TestEvalBoolCase {
                input: "2 > 1",
                output: true,
            },
            TestEvalBoolCase {
                input: "!true",
                output: false,
            },
            TestEvalBoolCase {
                input: "!false",
                output: true,
            },
            TestEvalBoolCase {
                input: "!!true",
                output: true,
            },
            TestEvalBoolCase {
                input: "!!false",
                output: false,
            },
            TestEvalBoolCase {
                input: "true == true",
                output: true,
            },
            TestEvalBoolCase {
                input: "3 == 3",
                output: true,
            },
        ];
        for test in tests {
            assert_eq!(
                Object::Boolean(test.output),
                *eval_expression_statement(test.input)
            );
        }
    }

    struct TestEvalAnyCase {
        input: &'static str,
        output: Object,
    }
    #[test]
    fn test_if_expression() {
        let tests: Vec<TestEvalAnyCase> = vec![
            TestEvalAnyCase {
                input: "if (true) { 5 }",
                output: Object::Integer(5),
            },
            TestEvalAnyCase {
                input: "if (3 > 2) { 65 }",
                output: Object::Integer(65),
            },
            TestEvalAnyCase {
                input: "if (true) {}",
                output: Object::Null,
            },
            TestEvalAnyCase {
                input: "if (false) {}",
                output: Object::Null,
            },
            TestEvalAnyCase {
                input: "if (false) { 5 } else {}",
                output: Object::Null,
            },
            TestEvalAnyCase {
                input: "if (false) { 2 } else { 3 }",
                output: Object::Integer(3),
            },
        ];
        for test in tests {
            assert_eq!(test.output, *eval_expression_statement(test.input));
        }
    }

    fn eval_expression_statement(input: &'static str) -> Rc<Object> {
        let mut lexer = lexer::new(input);
        let mut parser = parser::Parser::new(&mut lexer);
        let mut program = parser.parse_program().unwrap();
        let env = Environment::new();
        let statement: ast::Statement = program.statements.remove(0);
        if let ast::Statement::Expression { expression } = statement {
            eval::eval_expression(expression, &Rc::new(RefCell::new(env))).unwrap()
        } else {
            panic!("Expected expression statement")
        }
    }

    fn eval_program(input: &'static str) -> Rc<Object> {
        let mut lexer = lexer::new(input);
        let mut parser = parser::Parser::new(&mut lexer);
        let program = parser.parse_program().unwrap();
        let env = Environment::new();
        eval::eval_program(program, &Rc::new(RefCell::new(env)))
            .unwrap()
            .unwrap()
    }

    #[test]
    fn test_return_statement() {
        let tests: Vec<TestEvalAnyCase> = vec![
            TestEvalAnyCase {
                input: "return 3;",
                output: Object::Integer(3),
            },
            TestEvalAnyCase {
                input: "return 10; 9;",
                output: Object::Integer(10),
            },
            TestEvalAnyCase {
                input: "return 2*5; 9;",
                output: Object::Integer(10),
            },
            TestEvalAnyCase {
                input: "9; return 2+5; 9;",
                output: Object::Integer(7),
            },
            TestEvalAnyCase {
                input: "
                if (10 > 1) {
                    if (10 > 1) {
                        return 10; 
                    }
                    return 1;
                }
                ",
                output: Object::Integer(10),
            },
        ];
        for test in tests {
            assert_eq!(test.output, *eval_program(test.input));
        }
    }

    struct TestErrorCase {
        input: &'static str,
        error_message: &'static str,
    }
    #[test]
    fn test_errors() {
        let tests: Vec<TestErrorCase> = vec![
            TestErrorCase {
                input: "5 + true;",
                error_message: "Eval error: Cannot evaluate infix expression 5 + true",
            },
            TestErrorCase {
                input: "5 + true; 5;",
                error_message: "Eval error: Cannot evaluate infix expression 5 + true",
            },
            TestErrorCase {
                input: "-true",
                error_message: "Eval error: The prefix - cannot appear before type Boolean",
            },
            TestErrorCase {
                input: "true + false;",
                error_message: "Eval error: Cannot evaluate infix expression true + false",
            },
            TestErrorCase {
                input: "5; true + false; 5;",
                error_message: "Eval error: Cannot evaluate infix expression true + false",
            },
            TestErrorCase {
                input: "if (10 > 1) { true + false; }",
                error_message: "Eval error: Cannot evaluate infix expression true + false",
            },
            TestErrorCase {
                input: "!5;",
                error_message: "Eval error: The prefix ! cannot appear before type Integer",
            },
            TestErrorCase {
                input: "foobar;",
                error_message: "Eval error: The identifier 'foobar' has not been bound",
            },
        ];
        for test in tests {
            let mut lexer = lexer::new(test.input);
            let mut parser = parser::Parser::new(&mut lexer);
            let program = parser.parse_program().unwrap();
            let env = Environment::new();
            let evaluation_result = eval::eval_program(program, &Rc::new(RefCell::new(env)))
                .map_err(|e| errors::MonkeyError::Eval(e))
                .unwrap_err();
            assert_eq!(
                evaluation_result.to_string(),
                String::from(test.error_message)
            );
        }
    }

    #[test]
    fn test_let_statements() {
        let tests: Vec<TestEvalAnyCase> = vec![
            TestEvalAnyCase {
                input: "let a = 5; a;",
                output: Object::Integer(5),
            },
            TestEvalAnyCase {
                input: "let a = 5 * 5; a;",
                output: Object::Integer(25),
            },
            TestEvalAnyCase {
                input: "let a = 5; let b = a; b;",
                output: Object::Integer(5),
            },
            TestEvalAnyCase {
                input: "let a = 5; let b = a; let c = a + b + 5; c;",
                output: Object::Integer(15),
            },
            // Annoying but that seems to be the behaviour we've gone for for now.
            TestEvalAnyCase {
                input: "
                    if (true) {
                        let a = 3;
                    }
                    a;
                ",
                output: Object::Integer(3),
            },
        ];
        for test in tests {
            let result = eval_program(test.input);
            assert_eq!(*result, test.output);
        }
    }

    #[test]
    fn test_fn_application() {
        let tests: Vec<TestEvalAnyCase> = vec![
            TestEvalAnyCase {
                input: "let identity = fn(x) { x; }; identity(5);",
                output: Object::Integer(5),
            },
            TestEvalAnyCase {
                input: "let identity = fn(x) { return x; }; identity(5);",
                output: Object::Integer(5),
            },
            TestEvalAnyCase {
                input: "let double = fn(x) { x * 2; }; double(5);",
                output: Object::Integer(10),
            },
            TestEvalAnyCase {
                input: "let add = fn(x, y) { x + y; }; add(5, 5);",
                output: Object::Integer(10),
            },
            TestEvalAnyCase {
                input: "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));",
                output: Object::Integer(20),
            },
            TestEvalAnyCase {
                input: "fn(x) { x; }(5)",
                output: Object::Integer(5),
            },
            // Check closure can capture its env
            TestEvalAnyCase {
                input: "let a = 3; let b = fn() { a; }; b();",
                output: Object::Integer(3),
            },
            // Check closure doesn't overwrite parent env
            TestEvalAnyCase {
                input: "let a = 3; fn() { let a = 5; }(); a;",
                output: Object::Integer(3),
            },
            TestEvalAnyCase {
                input: "
                let times_by_five = fn() { 
                    let a = 5;
                    fn(x) {
                        x * a
                    };
                }();
                times_by_five(3);
                ", // this works even though the env the inner fn was defined in no longer exists!
                output: Object::Integer(15),
            },
            TestEvalAnyCase {
                input: "let multiply = fn(x) { fn(y) { x * y }; }; multiply(3)(5);",
                output: Object::Integer(15),
            },
        ];
        for test in tests {
            let result = eval_program(test.input);
            assert_eq!(*result, test.output);
        }
    }
}
