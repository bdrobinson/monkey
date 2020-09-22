use appendlist::AppendList;
use core::cell::RefCell;
use io::BufRead;
use monkey::{ast, errors, eval, lexer, object, parser};
use object::environment;
use std::io;
use std::rc::Rc;

const PROMPT: &str = ">> ";

pub fn start(
    input: &mut dyn io::BufRead,
    output: &mut dyn io::Write,
    error: &mut dyn io::Write,
) -> Result<(), io::Error> {
    output.write_all(b"Welcome to the Monkey REPL!\n")?;
    output.write_all(b"Type some code!\n")?;
    output.write_all(PROMPT.as_bytes())?;
    output.flush()?;

    // The objects will hold references to the ast nodes so each line of
    // the program must live as long as the env. We have to use an
    // append-only list as Vec is mutable so rust can't guarantee the
    // program we're pointing to won't get invalidated down the line.
    // This still doesn't feel like a great solution though...
    let program_bank = AppendList::<ast::Program>::new();
    let env = Rc::new(RefCell::new(environment::Environment::new()));
    for line_result in input.lines() {
        let line = line_result.unwrap();
        program_bank.push(parse_line(&line).unwrap());
        let program = program_bank.iter().last().unwrap();
        match eval_line(program, Rc::clone(&env)) {
            Ok(evaluated) => {
                if let Some(obj) = evaluated {
                    output.write_all(format!("{}\n", obj).as_bytes())?;
                } else {
                    output.write_all(b"\n")?;
                }
            }
            Err(message) => {
                error.write_all(format!("{}\n", message).as_bytes())?;
            }
        }
        output.write_all(PROMPT.as_bytes())?;
        output.flush()?;
    }
    Ok(())
}

fn eval_line<'a>(
    program: &'a ast::Program,
    env: Rc<RefCell<environment::Environment<'a>>>,
) -> Result<Option<Rc<object::Object<'a>>>, errors::MonkeyError>
where
{
    let object = eval::eval_program(&program, env).map_err(errors::MonkeyError::Eval)?;
    Ok(object)
}

fn parse_line(line: &str) -> Result<ast::Program, errors::MonkeyError> {
    let line = line.trim();
    let mut lexer = lexer::new(line);
    let mut parser = parser::Parser::new(&mut lexer);
    let program = parser
        .parse_program()
        .map_err(errors::MonkeyError::Parser)?;
    Ok(program)
}
