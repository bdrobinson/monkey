mod lexer;
mod repl;
mod token;

use std::io;

fn main() {
    repl::start(&mut io::stdin().lock(), &mut io::stdout()).expect("Repl failed");
}
