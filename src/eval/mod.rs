use crate::ast;
use crate::object::Object;
mod test;

pub fn eval_expression(expression: ast::Expression) -> Result<Object, String> {
    match expression {
        ast::Expression::IntegerLiteral { value } => Ok(Object::Integer(value)),
        ast::Expression::Infix {
            left,
            operator,
            right,
        } => {
            let left = eval_expression(*left)?;
            let right = eval_expression(*right)?;
            if let Object::Integer(left) = left {
                if let Object::Integer(right) = right {
                    return Ok(arithmetic(left, operator, right));
                }
            }
            unimplemented!();
        }
        ast::Expression::Boolean { value } => Ok(Object::Boolean(value)),
        ast::Expression::Prefix { operator, right } => {
            let object = eval_expression(*right)?;
            match (&operator, &object) {
                (ast::PrefixOperator::Minus, Object::Integer(value)) => Ok(Object::Integer(-value)),
                (ast::PrefixOperator::Bang, Object::Boolean(value)) => Ok(Object::Boolean(!value)),
                _ => Err(format!(
                    "The prefix {} cannot appear before type {}",
                    operator,
                    object.type_name()
                )),
            }
        }
        _ => {
            unimplemented!();
        }
    }
}

fn eval_statement(statement: ast::Statement) -> Result<Option<Object>, String> {
    match statement {
        ast::Statement::Expression { expression } => {
            let object = eval_expression(expression)?;
            Ok(Some(object))
        }
        _ => {
            unimplemented!();
        }
    }
}

pub fn eval_program(program: ast::Program) -> Result<Option<Object>, String> {
    let mut result: Option<Object> = None;
    for statement in program.statements {
        result = eval_statement(statement)?;
    }
    Ok(result)
}

fn arithmetic(left: i64, op: ast::InfixOperator, right: i64) -> Object {
    match op {
        ast::InfixOperator::Plus => Object::Integer(left + right),
        ast::InfixOperator::Minus => Object::Integer(left - right),
        ast::InfixOperator::Multiply => Object::Integer(left * right),
        ast::InfixOperator::Divide => Object::Integer(left / right),
        ast::InfixOperator::Gt => Object::Boolean(left > right),
        ast::InfixOperator::Lt => Object::Boolean(left < right),
        ast::InfixOperator::Eq => Object::Boolean(left == right),
        ast::InfixOperator::NotEq => Object::Boolean(left != right),
    }
}
