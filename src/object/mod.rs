pub mod environment;

use crate::ast;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
    ReturnValue(Rc<Object>),
    Function {
        parameter_names: Vec<String>,
        body: ast::BlockStatement,
        env: environment::Environment,
    },
}
impl Object {
    pub fn type_name(&self) -> String {
        let string = match self {
            Object::Integer(_) => "Integer",
            Object::Boolean(_) => "Boolean",
            Object::Null => "Null",
            Object::ReturnValue(_) => "Return value",
            Object::Function {
                parameter_names: _,
                body: _,
                env: _,
            } => "Function",
        };
        String::from(string)
    }
}

impl PartialEq for Object {
    fn eq(&self, rhs: &Object) -> bool {
        match (self, rhs) {
            (Object::Integer(l), Object::Integer(r)) => l == r,
            (Object::Boolean(l), Object::Boolean(r)) => l == r,
            (Object::Null, Object::Null) => true,
            (Object::ReturnValue(l), Object::ReturnValue(r)) => l == r,
            _ => false,
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let repr: String = match self {
            Object::Integer(value) => value.to_string(),
            Object::Boolean(value) => value.to_string(),
            Object::Null => String::from("null"),
            Object::ReturnValue(obj) => String::from(format!("Return value: {}", obj)),
            Object::Function {
                parameter_names: _,
                body: _,
                env: _,
            } => String::from("Function"),
        };
        write!(f, "{}", repr)?;
        Ok(())
    }
}
