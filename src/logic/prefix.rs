use std::fmt;
use std::rc::Rc;

use crate::object::Object;

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

pub fn eval_prefix<'a>(
    operand: Rc<Object<'a>>,
    operator: &PrefixOperator,
) -> Result<Object<'a>, String> {
    match (operator, &*operand) {
        (PrefixOperator::Minus, Object::Integer(value)) => Ok(Object::Integer(-value)),
        (PrefixOperator::Bang, Object::Boolean(value)) => Ok(Object::Boolean(!value)),
        _ => Err(format!(
            "The prefix {} cannot appear before type {}",
            operator,
            operand.type_name()
        )),
    }
}
