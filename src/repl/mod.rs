use io::BufRead;
use std::io;

use crate::{lexer, parser};

const PROMPT: &str = ">> ";

pub fn start(input: &mut dyn io::BufRead, output: &mut dyn io::Write) -> Result<(), io::Error> {
    output.write("Welcome to the Monkey REPL!\n".as_bytes())?;
    output.write("Type some code!\n".as_bytes())?;
    output.write(PROMPT.as_bytes())?;
    output.flush()?;
    for line_result in input.lines() {
        let line = line_result.unwrap();
        let line = line.trim();
        let mut lexer = lexer::new(line);
        let mut parser = parser::Parser::new(&mut lexer);
        let program = parser.parse_program();
        match program {
            Ok(program) => {
                output.write(format!("{}\n", program).as_bytes())?;
            }
            Err(message) => {
                output.write(format!("Parsing failed: {}\n", message).as_bytes())?;
            }
        }
        output.write(PROMPT.as_bytes())?;
        output.flush()?;
    }
    Ok(())
}
