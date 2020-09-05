mod repl;

use clap::Clap;
use core::cell::RefCell;
use monkey::{eval, lexer, object::environment, parser};
use std::fs;
use std::io;
use std::rc::Rc;

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
struct Opts {
    source_file: Option<String>,
}
fn main() {
    let opts: Opts = Opts::parse();

    if let Some(source_file) = opts.source_file {
        let source_code = fs::read_to_string(source_file).unwrap();
        run_program(source_code)
    } else {
        repl::start(
            &mut io::stdin().lock(),
            &mut io::stdout(),
            &mut io::stderr(),
        )
        .expect("Repl failed");
    }
}

fn run_program(source_code: String) {
    let env = Rc::new(RefCell::new(environment::Environment::new()));
    let mut lexer = lexer::new(&source_code);
    let mut parser = parser::Parser::new(&mut lexer);
    let program = parser.parse_program().unwrap();
    eval::eval_program(&program, env).unwrap();
}
