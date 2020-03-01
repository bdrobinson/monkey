use io::BufRead;
use std::io;

use crate::token::Token;

const PROMPT: &str = ">> ";

pub fn start(input: &mut dyn io::BufRead, output: &mut dyn io::Write) -> Result<(), io::Error> {
    output.write("Welcome to the Monkey REPL!\n".as_bytes())?;
    output.write("Type some code!\n".as_bytes())?;
    output.write(PROMPT.as_bytes())?;
    output.flush()?;
    for line_result in input.lines() {
        let line = line_result.unwrap();
        let line = line.trim();
        let mut lexer = crate::lexer::new(line);
        loop {
            let tok = lexer.next_token();
            match tok {
                Token::Eof => {
                    break;
                }
                Token::Illegal { literal } => {
                    panic!("Got an illegal token: {}", literal);
                }
                _ => {
                    output.write(format!("{:?}\n", tok).as_bytes())?;
                }
            }
        }
        output.write(PROMPT.as_bytes())?;
        output.flush()?;
    }
    Ok(())
}
