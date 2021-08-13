mod repl;

use clap::Clap;
use core::cell::RefCell;
use monkey::{eval, lexer, object::environment, parser};
use std::fs;
use std::io;
use std::rc::Rc;

enum EvalMethod {
    Interpreter,
    Vm,
}
impl std::str::FromStr for EvalMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "interpreter" => Ok(Self::Interpreter),
            "vm" => Ok(Self::Vm),
            _ => Err("Not a valid eval method".into()),
        }
    }
}

#[derive(Clap)]
struct Opts {
    source_file: Option<String>,
    #[clap(long)]
    use_interpreter: Option<bool>,
}

fn main() {
    let opts: Opts = Opts::parse();
    let use_interpreter = opts.use_interpreter.unwrap_or(true);
    if let Some(source_file) = opts.source_file {
        let source_code = fs::read_to_string(source_file).unwrap();
        run_program(source_code, use_interpreter)
    } else {
        repl::start(
            &mut io::stdin().lock(),
            &mut io::stdout(),
            &mut io::stderr(),
            use_interpreter,
        )
        .expect("Repl failed");
    }
}

fn run_program(source_code: String, use_interpreter: bool) {
    let env = Rc::new(RefCell::new(environment::Environment::new()));
    let mut lexer = lexer::new(&source_code);
    let mut parser = parser::Parser::new(&mut lexer);
    let program = parser.parse_program().unwrap();
    if use_interpreter {
        eval::eval_program(&program, env).unwrap();
    } else {
        unimplemented!("VM is not yet supported")
    }
}
