mod repl;

use std::io;

fn main() {
    repl::start(
        &mut io::stdin().lock(),
        &mut io::stdout(),
        &mut io::stderr(),
    )
    .expect("Repl failed");
}
