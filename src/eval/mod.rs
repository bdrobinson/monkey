use crate::ast;
use crate::object::{environment::Environment, Object};
use std::rc::Rc;
mod test;

pub fn eval_expression(
    expression: ast::Expression,
    env: &mut Environment,
) -> Result<Rc<Object>, String> {
    match expression {
        ast::Expression::IntegerLiteral { value } => Ok(Rc::new(Object::Integer(value))),
        ast::Expression::Infix {
            left,
            operator,
            right,
        } => {
            let left = eval_expression(*left, env)?;
            let right = eval_expression(*right, env)?;
            eval_infix(left.clone(), operator, right.clone()).map(|exp| Rc::new(exp))
        }
        ast::Expression::Boolean { value } => Ok(Rc::new(Object::Boolean(value))),
        ast::Expression::Prefix { operator, right } => {
            let object = eval_expression(*right, env)?;
            match (&operator, &*object) {
                (ast::PrefixOperator::Minus, Object::Integer(value)) => {
                    Ok(Rc::new(Object::Integer(-value)))
                }
                (ast::PrefixOperator::Bang, Object::Boolean(value)) => {
                    Ok(Rc::new(Object::Boolean(!value)))
                }
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
            let condition = eval_expression(*condition, env)?;
            let condition = if let Object::Boolean(value) = *condition {
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
            let evaluated_block = eval_statements(block_to_eval.statements, env)?;
            Ok(evaluated_block.unwrap_or(Rc::new(Object::Null)))
        }
        ast::Expression::Identifier { value } => {
            let obj = env.get(&value)?;
            Ok(obj.clone())
        }
        _ => {
            unimplemented!();
        }
    }
}

fn eval_statements(
    statements: Vec<ast::Statement>,
    env: &mut Environment,
) -> Result<Option<Rc<Object>>, String> {
    let mut result: Option<Rc<Object>> = None;
    for statement in statements {
        result = eval_statement(statement, env)?;
        if let Some(evaluated_statement) = &result {
            if matches!(&**evaluated_statement, Object::ReturnValue(_)) {
                break;
            }
        }
    }
    Ok(result)
}

fn eval_statement(
    statement: ast::Statement,
    env: &mut Environment,
) -> Result<Option<Rc<Object>>, String> {
    match statement {
        ast::Statement::Expression { expression } => {
            let object = eval_expression(expression, env)?;
            Ok(Some(object))
        }
        ast::Statement::Return { value } => {
            let contained_value = eval_expression(value, env)?;
            Ok(Some(Rc::new(Object::ReturnValue(contained_value))))
        }
        ast::Statement::Let { name, right } => {
            let right_obj = eval_expression(right, env)?;
            env.set(&name, right_obj);
            Ok(None)
        }
    }
}

pub fn eval_program(
    program: ast::Program,
    env: &mut Environment,
) -> Result<Option<Rc<Object>>, String> {
    let evaluated = eval_statements(program.statements, env)?;
    let evaluated: Option<Rc<Object>> = evaluated.map(|o| {
        if let Object::ReturnValue(value) = &*o {
            Rc::clone(value)
        } else {
            o
        }
    });
    Ok(evaluated)
}

fn eval_infix(
    left: Rc<Object>,
    op: ast::InfixOperator,
    right: Rc<Object>,
) -> Result<Object, String> {
    match (&*left, &op, &*right) {
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
