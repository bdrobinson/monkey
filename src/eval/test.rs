#[cfg(test)]
mod test {

    use crate::ast;
    use crate::eval;
    use crate::lexer;
    use crate::object::Object;
    use crate::parser;

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
                eval_expression_statement(test.input)
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
                eval_expression_statement(test.input)
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
            assert_eq!(test.output, eval_expression_statement(test.input));
        }
    }

    fn eval_expression_statement(input: &'static str) -> Object {
        let mut lexer = lexer::new(input);
        let mut parser = parser::Parser::new(&mut lexer);
        let mut program = parser.parse_program().unwrap();
        let statement: ast::Statement = program.statements.remove(0);
        if let ast::Statement::Expression { expression } = statement {
            eval::eval_expression(expression).unwrap()
        } else {
            panic!("Expected expression statement")
        }
    }

    // func testEval(input string) object.Object { l := lexer.New(input)
    // p := parser.New(l)
    // program := p.ParseProgram()
    // return Eval(program) }
    // func testIntegerObject(t *testing.T, obj object.Object, expected int64) bool { result, ok := obj.(*object.Integer)
    // if !ok {
    // t.Errorf("object is not Integer. got=%T (%+v)", obj, obj)
    // return false }
    // if result.Value != expected {
    // t.Errorf("object has wrong value. got=%d, want=%d",
    // result.Value, expected)
    // return false }
    // return true }
}
