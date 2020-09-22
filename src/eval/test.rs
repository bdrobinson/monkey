use crate::errors;
use crate::eval;
use crate::lexer;
use crate::object::{environment::Environment, Object};
use crate::parser;
use core::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_eval_integer_expression() {
    let tests: Vec<TestCase> = vec![
        TestCase::int("5", 5),
        TestCase::int("10", 10),
        TestCase::int("10 + 15", 25),
        TestCase::int("2 - 12", -10),
        TestCase::int("5 + 10 / 2 ", 10),
        TestCase::int("-5", -5),
    ];
    for test in tests {
        run_test_case(test);
    }
}

struct TestCase<'a> {
    code: &'a str,
    value: TestCaseValue,
}
impl<'a> TestCase<'a> {
    fn int(code: &'a str, i: i64) -> TestCase<'a> {
        TestCase {
            code,
            value: TestCaseValue::Int(i),
        }
    }
    fn bool(code: &'a str, i: bool) -> TestCase<'a> {
        TestCase {
            code,
            value: TestCaseValue::Bool(i),
        }
    }
    fn null(code: &'a str) -> TestCase<'a> {
        TestCase {
            code,
            value: TestCaseValue::Null,
        }
    }
    fn string(code: &'a str, string: String) -> TestCase<'a> {
        TestCase {
            code,
            value: TestCaseValue::Str(string),
        }
    }
}

fn run_test_case(case: TestCase) {
    let mut lexer = lexer::new(case.code);
    let mut parser = parser::Parser::new(&mut lexer);
    let program = parser.parse_program().unwrap();
    let env = Environment::new();
    let result = eval::eval_program(&program, Rc::new(RefCell::new(env)))
        .unwrap()
        .unwrap();
    let expected = match case.value {
        TestCaseValue::Bool(b) => Object::Boolean(b),
        TestCaseValue::Int(i) => Object::Integer(i),
        TestCaseValue::Null => Object::Null,
        TestCaseValue::Str(st) => Object::String(st),
    };
    assert_eq!(*result, expected);
    println!("{}", program);
}

enum TestCaseValue {
    Int(i64),
    Bool(bool),
    Null,
    Str(String),
}

#[test]
fn test_eval_boolean_expression() {
    let tests: Vec<TestCase> = vec![
        TestCase::bool("true", true),
        TestCase::bool("false", false),
        TestCase::bool("2 < 3", true),
        TestCase::bool("2 < 1", false),
        TestCase::bool("2 > 3", false),
        TestCase::bool("2 > 1", true),
        TestCase::bool("!true", false),
        TestCase::bool("!false", true),
        TestCase::bool("!!true", true),
        TestCase::bool("!!false", false),
        TestCase::bool("true == true", true),
        TestCase::bool("3 == 3", true),
    ];
    for test in tests {
        run_test_case(test);
    }
}

#[test]
fn test_if_expression() {
    let tests: Vec<TestCase> = vec![
        TestCase::int("if (true) { 5 }", 5),
        TestCase::int("if (3 > 2) { 65 }", 65),
        TestCase::null("if (true) {}"),
        TestCase::null("if (false) {}"),
        TestCase::null("if (false) { 5 } else {}"),
        TestCase::int("if (false) { 2 } else { 3 }", 3),
        TestCase::int(
            "
                let a = 3;
                if (true) { 
                    let a = 5;
                }
                a
                ",
            3,
        ),
        TestCase::int(
            "
                let a = 3;
                if (false) {} else { 
                    let a = 5;
                }
                a
                ",
            3,
        ),
    ];
    for test in tests {
        run_test_case(test);
    }
}

#[test]
fn test_return_statement() {
    let tests: Vec<TestCase> = vec![
        TestCase::int("return 3;", 3),
        TestCase::int("return 10; 9;", 10),
        TestCase::int("return 2*5; 9;", 10),
        TestCase::int("9; return 2+5; 9;", 7),
        TestCase::int(
            "
                if (10 > 1) {
                    if (10 > 1) {
                        return 10; 
                    }
                    return 1;
                }
                ",
            10,
        ),
    ];
    for test in tests {
        run_test_case(test);
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
        let evaluation_result = eval::eval_program(&program, Rc::new(RefCell::new(env)))
            .map_err(errors::MonkeyError::Eval)
            .unwrap_err();
        assert_eq!(
            evaluation_result.to_string(),
            String::from(test.error_message)
        );
    }
}

#[test]
fn test_let_statements() {
    let tests: Vec<TestCase> = vec![
        TestCase::int("let a = 5; a;", 5),
        TestCase::int("let a = 5 * 5; a;", 25),
        TestCase::int("let a = 5; let b = a; b;", 5),
        TestCase::int("let a = 5; let b = a; let c = a + b + 5; c;", 15),
    ];
    for test in tests {
        run_test_case(test);
    }
}

#[test]
fn test_fn_application() {
    let tests: Vec<TestCase> = vec![
        TestCase::int("let identity = fn(x) { x; }; identity(5);", 5),
        TestCase::int("let identity = fn(x) { return x; }; identity(5);", 5),
        TestCase::int("let double = fn(x) { x * 2; }; double(5);", 10),
        TestCase::int("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
        TestCase::int("let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));", 20),
        TestCase::int("fn(x) { x; }(5)", 5),
        // Check closure can capture its env
        TestCase::int("let a = 3; let b = fn() { a; }; b();", 3),
        // Check closure doesn't overwrite parent env
        TestCase::int("let a = 3; fn() { let a = 5; }(); a;", 3),
        TestCase::int(
            "
                let times_by_five = fn() { 
                    let a = 5;
                    fn(x) {
                        x * a
                    };
                }();
                times_by_five(3);
                ", // this works even though the env the inner fn was defined in no longer exists!
            15,
        ),
        TestCase::int(
            "let multiply = fn(x) { fn(y) { x * y }; }; multiply(3)(5);",
            15,
        ),
    ];
    for test in tests {
        run_test_case(test);
    }
}

#[test]
fn test_string_literals() {
    let tests: Vec<TestCase> = vec![
        TestCase::string("\"ahoy shipmates\";", String::from("ahoy shipmates")),
        TestCase::string(
            r#""hello" + " " + "everyone!""#,
            String::from("hello everyone!"),
        ),
    ];
    for test in tests {
        run_test_case(test);
    }
}

#[test]
fn test_block_expressions() {
    let tests: Vec<TestCase> = vec![
        TestCase::int("let a = { 2; 3; }; a", 3),
        TestCase::int(
            "
            let a = 3;
            { 
                let a = 5;
            }
            a
            ",
            3,
        ),
    ];
    for test in tests {
        run_test_case(test);
    }
}

#[test]
fn test_builtins() {
    let tests: Vec<TestCase> = vec![TestCase::int("len(\"ahoy\")", 4)];
    for test in tests {
        run_test_case(test);
    }
}
