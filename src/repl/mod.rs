use io::BufRead;
use std::io;
use std::rc::Rc;

use crate::{eval, lexer, object, parser};
use object::environment;

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
    let mut env = environment::Environment::new();
    for line_result in input.lines() {
        let line = line_result.unwrap();
        match eval_line(line, &mut env) {
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
    env: &mut environment::Environment,
) -> Result<Option<Rc<object::Object>>, String> {
    let line = line.trim();
    let mut lexer = lexer::new(line);
    let mut parser = parser::Parser::new(&mut lexer);
    let program = parser.parse_program()?;
    let object = eval::eval_program(program, env)?;
    Ok(object)
}
