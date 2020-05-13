use crate::ast;
use crate::object::{environment::Environment, Object};
use core::cell::RefCell;
use std::rc::Rc;

mod test;

#[derive(Debug)]
pub enum EvalError {
    Misc(String),
}

pub fn eval_expression(
    expression: ast::Expression,
    env: &Rc<RefCell<Environment>>,
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
            eval_infix(left, operator, right).map(|exp| Rc::new(exp))
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
            let obj = read_from_env(&env.borrow(), &value)?;
            Ok(obj.clone())
        }
        ast::Expression::FnLiteral { param_names, body } => Ok(Rc::new(Object::Function {
            body: body.clone(),
            parameter_names: param_names.clone(),
            env: Rc::clone(&env),
        })),
        ast::Expression::CallExpression { left, arguments } => {
            let left_evaluated = eval_expression(*left, env)?;
            let evaluated_arguments = eval_expressions(arguments, env)?;
            match &*left_evaluated {
                Object::Function {
                    parameter_names,
                    body,
                    env,
                } => call_function(&evaluated_arguments, &parameter_names, body.clone(), env),
                _ => Err(format!("Cannot call {}", left_evaluated)),
            }
        }
        ast::Expression::StringLiteral { value } => Ok(Rc::new(Object::String(value))),
    }
}

fn eval_expressions(
    expressions: Vec<ast::Expression>,
    env: &Rc<RefCell<Environment>>,
) -> Result<Vec<Rc<Object>>, String> {
    // TODO: use iterators
    let mut results: Vec<Rc<Object>> = vec![];
    for expression in expressions {
        let obj = eval_expression(expression, &env)?;
        results.push(obj);
    }
    Ok(results)
}

fn call_function(
    args: &Vec<Rc<Object>>,
    expected_param_names: &Vec<String>,
    body: ast::BlockStatement,
    parent_env: &Rc<RefCell<Environment>>,
) -> Result<Rc<Object>, String> {
    if args.len() != expected_param_names.len() {
        return Err(format!(
            "Expected {} args, got {}",
            expected_param_names.len(),
            args.len()
        ));
    }
    let mut call_env = Environment::new_enclosed(Rc::clone(&parent_env));

    for (name, obj) in expected_param_names.iter().zip(args) {
        call_env.set(name, Rc::clone(obj));
    }
    let result = eval_statements(body.statements, &Rc::new(RefCell::new(call_env)))?;
    Ok(result.unwrap_or(Rc::new(Object::Null)))
}

fn eval_statements(
    statements: Vec<ast::Statement>,
    env: &Rc<RefCell<Environment>>,
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
    env: &Rc<RefCell<Environment>>,
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
            env.borrow_mut().set(&name, right_obj);
            Ok(None)
        }
    }
}

pub fn eval_program(
    program: ast::Program,
    env: &Rc<RefCell<Environment>>,
) -> Result<Option<Rc<Object>>, EvalError> {
    let evaluated = eval_statements(program.statements, env).map_err(|e| EvalError::Misc(e))?;
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
        (Object::String(left), ast::InfixOperator::Plus, Object::String(right)) => {
            Ok(Object::String(format!("{}{}", left, right)))
        }
        (left, op, right) => Err(format!(
            "Cannot evaluate infix expression {} {} {}",
            left, op, right
        )),
    }
}

fn read_from_env(env: &Environment, identifier: &str) -> Result<Rc<Object>, String> {
    env.get(identifier).ok_or(String::from(format!(
        "The identifier '{}' has not been bound",
        identifier
    )))
}
