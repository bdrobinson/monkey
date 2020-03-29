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
    Identifier(IdentifierExpression),
    IntegerLiteral(IntegerLiteralExpression),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub name: IdentifierExpression,
    // value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    // pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct IdentifierExpression {
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct IntegerLiteralExpression {
    pub value: i64,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
    pub expression: Expression,
}
