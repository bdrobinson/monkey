use crate::object;
use object::Object;
use std::fmt;
use std::rc::Rc;

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

pub fn eval_infix<'a>(
    left: Rc<Object<'a>>,
    op: &InfixOperator,
    right: Rc<Object<'a>>,
) -> Result<Object<'a>, String> {
    match (&*left, &op, &*right) {
        (_, InfixOperator::Eq, _) => Ok(Object::Boolean(left == right)),
        (_, InfixOperator::NotEq, _) => Ok(Object::Boolean(left != right)),
        (Object::Integer(left), InfixOperator::Plus, Object::Integer(right)) => {
            Ok(Object::Integer(left + right))
        }
        (Object::Integer(left), InfixOperator::Minus, Object::Integer(right)) => {
            Ok(Object::Integer(left - right))
        }
        (Object::Integer(left), InfixOperator::Multiply, Object::Integer(right)) => {
            Ok(Object::Integer(left * right))
        }
        (Object::Integer(left), InfixOperator::Divide, Object::Integer(right)) => {
            Ok(Object::Integer(left / right))
        }
        (Object::Integer(left), InfixOperator::Gt, Object::Integer(right)) => {
            Ok(Object::Boolean(left > right))
        }
        (Object::Integer(left), InfixOperator::Lt, Object::Integer(right)) => {
            Ok(Object::Boolean(left < right))
        }
        (Object::String(left), InfixOperator::Plus, Object::String(right)) => {
            Ok(Object::String(format!("{}{}", left, right)))
        }
        (left, op, right) => Err(format!(
            "Cannot evaluate infix expression {} {} {}",
            left, op, right
        )),
    }
}
