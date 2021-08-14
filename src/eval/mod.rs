use crate::object::{environment::Environment, Object};
use crate::{ast, logic};
use core::cell::RefCell;
use std::rc::Rc;

mod builtins;
#[cfg(test)]
mod test;

use builtins::get_builtin_fn;

#[derive(Debug)]
pub enum EvalError {
    Misc(String),
}

const EMPTY_BLOCK: ast::BlockStatement = ast::BlockStatement { statements: vec![] };
const EMPTY_BLOCK_REF: &ast::BlockStatement = &EMPTY_BLOCK;

pub fn eval_expression<'a>(
    expression: &'a ast::Expression,
    env: Rc<RefCell<Environment<'a>>>,
) -> Result<Rc<Object<'a>>, String> {
    match expression {
        ast::Expression::IntegerLiteral { value } => Ok(Rc::new(Object::Integer(*value))),
        ast::Expression::Infix {
            left,
            operator,
            right,
        } => {
            let left = eval_expression(left, Rc::clone(&env))?;
            let right = eval_expression(right, Rc::clone(&env))?;
            logic::eval_infix(left, operator, right).map(Rc::new)
        }
        ast::Expression::Boolean { value } => Ok(Rc::new(Object::Boolean(*value))),
        ast::Expression::Prefix { operator, right } => {
            let object = eval_expression(right, env)?;
            logic::eval_prefix(object, operator).map(Rc::new)
        }
        ast::Expression::If {
            condition,
            consequence,
            alternative,
        } => {
            let condition = eval_expression(condition, Rc::clone(&env))?;
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
                (false, None) => EMPTY_BLOCK_REF,
            };
            let evaluated_block =
                eval_statements_with_inner_env(&block_to_eval.statements, Rc::clone(&env))?;
            Ok(evaluated_block.unwrap_or_else(|| Rc::new(Object::Null)))
        }
        ast::Expression::Identifier { value } => {
            let obj = read_from_env(&env.borrow(), &value)?;
            Ok(obj)
        }
        ast::Expression::FnLiteral { param_names, body } => Ok(Rc::new(Object::Function {
            body,
            parameter_names: param_names.clone(),
            env: Rc::clone(&env),
        })),
        ast::Expression::CallExpression { left, arguments } => {
            let left_evaluated = eval_expression(left, Rc::clone(&env))?;
            let evaluated_arguments = eval_expressions(arguments, Rc::clone(&env))?;
            match &*left_evaluated {
                Object::Function {
                    parameter_names,
                    body,
                    env,
                } => call_function(evaluated_arguments, &parameter_names, body, Rc::clone(env)),
                Object::BuiltinFunction(builtin) => builtin.run(&evaluated_arguments),
                _ => Err(format!("Cannot call {}", left_evaluated)),
            }
        }
        ast::Expression::StringLiteral { value } => Ok(Rc::new(Object::String(value.clone()))),
        ast::Expression::Block { statements } => eval_statements_with_inner_env(statements, env)
            .map(|opt| opt.unwrap_or_else(|| Rc::new(Object::Null))),
    }
}

fn eval_expressions<'a>(
    expressions: &'a [ast::Expression],
    env: Rc<RefCell<Environment<'a>>>,
) -> Result<Vec<Rc<Object<'a>>>, String> {
    // TODO: use iterators
    let mut results: Vec<Rc<Object>> = vec![];
    for expression in expressions {
        let obj = eval_expression(expression, Rc::clone(&env))?;
        results.push(obj);
    }
    Ok(results)
}

fn call_function<'a>(
    args: Vec<Rc<Object<'a>>>,
    expected_param_names: &[String],
    body: &'a ast::BlockStatement,
    parent_env: Rc<RefCell<Environment<'a>>>,
) -> Result<Rc<Object<'a>>, String> {
    if args.len() != expected_param_names.len() {
        return Err(format!(
            "Expected {} args, got {}",
            expected_param_names.len(),
            args.len()
        ));
    }
    let mut call_env = Environment::new_enclosed(Rc::clone(&parent_env));

    for (name, obj) in expected_param_names.iter().zip(args) {
        call_env.set(name, Rc::clone(&obj));
    }
    let result = eval_statements(&body.statements, Rc::new(RefCell::new(call_env)))?;
    Ok(result.unwrap_or_else(|| Rc::new(Object::Null)))
}

fn eval_statements<'a>(
    statements: &'a [ast::Statement],
    env: Rc<RefCell<Environment<'a>>>,
) -> Result<Option<Rc<Object<'a>>>, String> {
    let mut result: Option<Rc<Object>> = None;
    for statement in statements {
        result = eval_statement(statement, Rc::clone(&env))?;
        if let Some(evaluated_statement) = &result {
            if matches!(&**evaluated_statement, Object::ReturnValue(_)) {
                break;
            }
        }
    }
    Ok(result)
}

fn eval_statements_with_inner_env<'a>(
    statements: &'a [ast::Statement],
    parent_env: Rc<RefCell<Environment<'a>>>,
) -> Result<Option<Rc<Object<'a>>>, String> {
    let inner_env = Rc::new(RefCell::new(Environment::new_enclosed(Rc::clone(
        &parent_env,
    ))));
    eval_statements(statements, inner_env)
}

fn eval_statement<'a>(
    statement: &'a ast::Statement,
    env: Rc<RefCell<Environment<'a>>>,
) -> Result<Option<Rc<Object<'a>>>, String> {
    match statement {
        ast::Statement::Expression { expression } => {
            let object = eval_expression(expression, Rc::clone(&env))?;
            Ok(Some(object))
        }
        ast::Statement::Return { value } => {
            let contained_value = eval_expression(value, Rc::clone(&env))?;
            Ok(Some(Rc::new(Object::ReturnValue(contained_value))))
        }
        ast::Statement::Let { name, right } => {
            let right_obj = eval_expression(right, Rc::clone(&env))?;
            env.borrow_mut().set(&name, right_obj);
            Ok(None)
        }
    }
}

pub fn eval_program<'prog, 'env>(
    program: &'prog ast::Program,
    env: Rc<RefCell<Environment<'env>>>,
) -> Result<Option<Rc<Object<'env>>>, EvalError>
where
    'prog: 'env,
{
    let evaluated = eval_statements(&program.statements, env).map_err(EvalError::Misc)?;
    let evaluated: Option<Rc<Object>> = evaluated.map(|o| {
        if let Object::ReturnValue(value) = &*o {
            Rc::clone(value)
        } else {
            o
        }
    });
    Ok(evaluated)
}

fn read_from_env<'a>(env: &Environment<'a>, identifier: &str) -> Result<Rc<Object<'a>>, String> {
    env.get(identifier)
        .or_else(|| get_builtin_fn(identifier).map(|f| Rc::new(Object::BuiltinFunction(f))))
        .ok_or_else(|| format!("The identifier '{}' has not been bound", identifier))
}
