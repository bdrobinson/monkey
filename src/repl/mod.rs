use appendlist::AppendList;
use core::cell::RefCell;
use io::BufRead;
use monkey::errors::MonkeyError;
use monkey::{ast, compiler, errors, eval, lexer, object, parser, vm};
use object::environment;
use std::io;
use std::rc::Rc;

const PROMPT: &str = ">> ";

pub fn start(
    input: &mut dyn io::BufRead,
    output: &mut dyn io::Write,
    error: &mut dyn io::Write,
    use_interpreter: bool,
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
        let line = line_result?;
        match eval_line(&line, &program_bank, Rc::clone(&env), use_interpreter) {
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
    line: &str,
    program_bank: &'a AppendList<ast::Program>,
    env: Rc<RefCell<environment::Environment<'a>>>,
    use_interpreter: bool,
) -> Result<Option<Rc<object::Object<'a>>>, errors::MonkeyError> {
    let program = parse_line(line)?;
    program_bank.push(program);
    let program = program_bank.iter().last().unwrap(); // there will always be an item in here as we just put one in.
    let object = if use_interpreter {
        eval::eval_program(&program, env).map_err(errors::MonkeyError::Eval)?
    } else {
        let bytecode = compiler::compile_program(program);
        let mut vm = vm::Vm::new(&bytecode);
        vm.run().map_err(|msg| MonkeyError::VmError(msg))?
    };
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
