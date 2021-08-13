use crate::eval::EvalError;
use crate::parser::ParserError;
use crate::vm::VmError;
use core::fmt::Display;

#[derive(Debug)]
pub enum MonkeyError {
    Parser(ParserError),
    Eval(EvalError),
    VmError(VmError),
}

impl Display for MonkeyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            MonkeyError::Parser(parser_err) => {
                let message = match parser_err {
                    ParserError::InvalidExpression { first_token } => format!(
                        "An expression cannot begin with token type {}",
                        first_token.token_type()
                    ),
                    ParserError::UnexpectedToken { expected, actual } => format!(
                        "Unexpected token. Expected {}, got {}",
                        expected,
                        actual.token_type()
                    ),
                };
                write!(f, "Parser error: {}", message)?;
                Ok(())
            }
            MonkeyError::Eval(eval_err) => {
                let EvalError::Misc(message) = eval_err;
                write!(f, "Eval error: {}", message)?;
                Ok(())
            }
            MonkeyError::VmError(err) => {
                let message = match err {
                    VmError::PopEmptyStack => "Cannot pop from an empty stack",
                    VmError::Misc(msg) => msg,
                };
                write!(f, "VM Error: {}", message)?;
                Ok(())
            }
        }
    }
}
