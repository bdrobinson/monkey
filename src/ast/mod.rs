use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Node {
    Program(Program),
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Identifier {
        value: String,
    },
    IntegerLiteral {
        value: i64,
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
        consequence: Box<BlockStatement>,
        alternative: Option<Box<BlockStatement>>,
    },
}
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_repr: String = match self {
            Expression::Identifier { value } => value.clone(),
            Expression::IntegerLiteral { value } => value.to_string(),
            Expression::Prefix { operator, right } => format!("({}{})", operator, right),
            Expression::Infix {
                left,
                operator,
                right,
            } => format!("({} {} {})", left, operator, right),
            Expression::Boolean { value } => value.to_string(),
            Expression::If {
                condition,
                consequence,
                alternative,
            } => String::from("if expression"),
        };
        write!(f, "{}", string_repr)
    }
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct BlockStatement {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub name: String,
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    // pub value: Expression,
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

#[derive(Debug, PartialEq)]
pub enum InfixOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Gt,
    Lt,
    Eq,
    NotEq,
}

impl fmt::Display for InfixOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            InfixOperator::Plus => "+",
            InfixOperator::Minus => "-",
            InfixOperator::Multiply => "*",
            InfixOperator::Divide => "/",
            InfixOperator::Gt => ">",
            InfixOperator::Lt => "<",
            InfixOperator::Eq => "==",
            InfixOperator::NotEq => "!=",
        };
        write!(f, "{}", string)
    }
}

#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
    pub expression: Expression,
}
