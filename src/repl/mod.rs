use crate::{errors, eval, lexer, object, parser};
use core::cell::RefCell;
use io::BufRead;
use object::environment;
use std::io;
use std::rc::Rc;

const PROMPT: &str = ">> ";

pub fn start(
    input: &mut dyn io::BufRead,
    output: &mut dyn io::Write,
    error: &mut dyn io::Write,
) -> Result<(), io::Error> {
    output.write("Welcome to the Monkey REPL!\n".as_bytes())?;
    output.write("Type some code!\n".as_bytes())?;
    output.write(PROMPT.as_bytes())?;
    output.flush()?;
    let env = Rc::new(RefCell::new(environment::Environment::new()));
    for line_result in input.lines() {
        let line = line_result.unwrap();
        match eval_line(line, &env) {
            Ok(evaluated) => {
                if let Some(obj) = evaluated {
                    output.write(format!("{}\n", obj).as_bytes())?;
                } else {
                    output.write("\n".as_bytes())?;
                }
            }
            Err(message) => {
                error.write(format!("{}\n", message).as_bytes())?;
            }
        }
        output.write(PROMPT.as_bytes())?;
        output.flush()?;
    }
    Ok(())
}

fn eval_line(
    line: String,
    env: &Rc<RefCell<environment::Environment>>,
) -> Result<Option<Rc<object::Object>>, errors::MonkeyError> {
    let line = line.trim();
    let mut lexer = lexer::new(line);
    let mut parser = parser::Parser::new(&mut lexer);
    let program = parser
        .parse_program()
        .map_err(|e| errors::MonkeyError::Parser(e))?;
    let object = eval::eval_program(program, env).map_err(|e| errors::MonkeyError::Eval(e))?;
    Ok(object)
}
