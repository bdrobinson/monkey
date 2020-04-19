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
            eval_infix(left, operator, right)
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
        ast::Expression::If {
            condition,
            consequence,
            alternative,
        } => {
            let condition = eval_expression(*condition)?;
            let condition = if let Object::Boolean(value) = condition {
                value
            } else {
                return Err(format!(
                    "The condition in an if statement must be a bool. Got {}",
                    condition.type_name()
                ));
            };
            // Pattern matching is cool.
            let block_to_eval = match (condition, alternative) {
                (true, _) => consequence,
                (false, Some(alternative)) => alternative,
                (false, None) => ast::BlockStatement { statements: vec![] },
            };
            let evaluated_block = eval_statements(block_to_eval.statements)?;
            Ok(evaluated_block.unwrap_or(Object::Null))
        }
        _ => {
            unimplemented!();
        }
    }
}

fn eval_statements(statements: Vec<ast::Statement>) -> Result<Option<Object>, String> {
    let mut result: Option<Object> = None;
    for statement in statements {
        let evaluated_statement_opt = eval_statement(statement)?;
        if let Some(evaluated_statement) = &evaluated_statement_opt {
            if let Object::ReturnValue(returned_value) = evaluated_statement {
                result = Some(*returned_value.clone());
                break;
            }
        }
        result = evaluated_statement_opt;
    }
    Ok(result)
}

fn eval_statement(statement: ast::Statement) -> Result<Option<Object>, String> {
    match statement {
        ast::Statement::Expression { expression } => {
            let object = eval_expression(expression)?;
            Ok(Some(object))
        }
        ast::Statement::Return { value } => {
            let contained_value = eval_expression(value)?;
            Ok(Some(Object::ReturnValue(Box::new(contained_value))))
        }
        _ => {
            unimplemented!();
        }
    }
}

pub fn eval_program(program: ast::Program) -> Result<Option<Object>, String> {
    eval_statements(program.statements)
}

fn eval_infix(left: Object, op: ast::InfixOperator, right: Object) -> Result<Object, String> {
    match (&left, &op, &right) {
        (_, ast::InfixOperator::Eq, _) => Ok(Object::Boolean(left == right)),
        (_, ast::InfixOperator::NotEq, _) => Ok(Object::Boolean(left != right)),
        (Object::Integer(left), ast::InfixOperator::Plus, Object::Integer(right)) => {
            Ok(Object::Integer(left + right))
        }
        (Object::Integer(left), ast::InfixOperator::Minus, Object::Integer(right)) => {
            Ok(Object::Integer(left - right))
        }
        (Object::Integer(left), ast::InfixOperator::Multiply, Object::Integer(right)) => {
            Ok(Object::Integer(left * right))
        }
        (Object::Integer(left), ast::InfixOperator::Divide, Object::Integer(right)) => {
            Ok(Object::Integer(left / right))
        }
        (Object::Integer(left), ast::InfixOperator::Gt, Object::Integer(right)) => {
            Ok(Object::Boolean(left > right))
        }
        (Object::Integer(left), ast::InfixOperator::Lt, Object::Integer(right)) => {
            Ok(Object::Boolean(left < right))
        }
        (left, op, right) => Err(format!(
            "Cannot evaluate infix expression {} {} {}",
            left, op, right
        )),
    }
}
