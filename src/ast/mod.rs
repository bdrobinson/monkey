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
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
}

#[derive(Debug, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct LetStatement {
    pub name: Identifier,
    // value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    // pub value: Expression,
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub value: String,
}

#[derive(Debug, PartialEq)]
pub struct IntegerLiteral {
    pub value: i64,
}

#[derive(Debug, PartialEq)]
pub struct ExpressionStatement {
    pub expression: Expression,
}
