use crate::logic;
use core::fmt::Display;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let { name: String, right: Expression },
    Return { value: Expression },
    Expression { expression: Expression },
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Statement::Let { name, right } => {
                write!(f, "let {} = {};", name, right)?;
            }
            Statement::Return { value } => {
                write!(f, "return {};", value)?;
            }
            Statement::Expression { expression } => {
                write!(f, "{};", expression)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier {
        value: String,
    },
    IntegerLiteral {
        value: i64,
    },
    StringLiteral {
        value: String,
    },
    Prefix {
        operator: PrefixOperator,
        right: Box<Expression>,
    },
    Infix {
        left: Box<Expression>,
        operator: InfixOperator,
        right: Box<Expression>,
    },
    Boolean {
        value: bool,
    },
    If {
        condition: Box<Expression>,
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    },
    FnLiteral {
        param_names: Vec<String>,
        body: BlockStatement,
    },
    CallExpression {
        left: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Block {
        statements: Vec<Statement>,
    },
}
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_repr: String = match &self {
            Expression::Identifier { value } => value.clone(),
            Expression::IntegerLiteral { value } => value.to_string(),
            Expression::StringLiteral { value } => value.clone(),
            Expression::Prefix { operator, right } => format!("({}{})", operator, right),
            Expression::Infix {
                left,
                operator,
                right,
            } => format!("({} {} {})", left, operator, right),
            Expression::Boolean { value } => value.to_string(),
            &Expression::If {
                condition,
                consequence,
                alternative,
            } => format!(
                "if ({}) {} {}",
                condition,
                consequence,
                alternative
                    .as_ref()
                    .map(|a| format!("else {}", a))
                    .unwrap_or_else(|| String::from(""))
            ),
            Expression::FnLiteral { param_names, body } => {
                format!("fn({}) {}", param_names.join(", "), body)
            }
            &Expression::CallExpression { left, arguments } => format!(
                "{}({})",
                left,
                arguments
                    .iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Expression::Block { .. } => String::from("Block statement"),
        };
        write!(f, "{}", string_repr)
    }
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for statement in &self.statements {
            write!(f, "{}", statement)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

impl Display for BlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{{")?;
        for statement in &self.statements {
            write!(f, "{}", statement)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum PrefixOperator {
    Bang,
    Minus,
}

impl fmt::Display for PrefixOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            PrefixOperator::Bang => "!",
            PrefixOperator::Minus => "-",
        };
        write!(f, "{}", string)
    }
}

pub use logic::InfixOperator;
