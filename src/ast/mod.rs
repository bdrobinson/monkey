use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let { name: String, right: Expression },
    Return { value: Expression },
    Expression { expression: Expression },
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
        consequence: BlockStatement,
        alternative: Option<BlockStatement>,
    },
    FnLiteral {
        param_names: Vec<String>,
        body: BlockStatement,
    },
    CallExpression {
        function: CallExpressionFunction,
        arguments: Vec<Expression>,
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
                condition: _,
                consequence: _,
                alternative: _,
            } => String::from("if expression"),
            Expression::FnLiteral {
                param_names: _,
                body: _,
            } => String::from("fn literal"),
            Expression::CallExpression {
                function: _,
                arguments: _,
            } => String::from("call expr"),
        };
        write!(f, "{}", string_repr)
    }
}

#[derive(Debug, PartialEq)]
pub enum CallExpressionFunction {
    // Bit annoying to have this duplicated logic but meh
    Literal {
        param_names: Vec<String>,
        body: BlockStatement,
    },
    Identifier {
        value: String,
    },
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
