mod ast;
mod errors;
mod eval;
mod lexer;
mod object;
mod parser;
mod repl;
mod token;

use std::io;

fn main() {
    repl::start(
        &mut io::stdin().lock(),
        &mut io::stdout(),
        &mut io::stderr(),
    )
    .expect("Repl failed");
}
