pub mod environment;
use crate::ast;
use core::cell::RefCell;
use std::fmt;
use std::fmt::Debug;
use std::rc::Rc;

pub trait BuiltinFunction: Debug {
    fn run(&self, arguments: &[Rc<Object>]) -> Result<Rc<Object>, String>;
}

#[derive(Debug)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    Null,
    ReturnValue(Rc<Object>),
    Function {
        parameter_names: Vec<String>,
        body: ast::BlockStatement,
        env: Rc<RefCell<environment::Environment>>,
    },
    BuiltinFunction(Box<dyn BuiltinFunction>),
}
impl Object {
    pub fn type_name(&self) -> String {
        let string = match self {
            Object::Integer(_) => "Integer",
            Object::Boolean(_) => "Boolean",
            Object::Null => "Null",
            Object::ReturnValue(_) => "Return value",
            Object::String(_) => "String",
            Object::Function { .. } => "Function",
            Object::BuiltinFunction(..) => "BuiltinFunction",
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
            (Object::String(l), Object::String(r)) => l == r,
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
            Object::String(value) => value.clone(),
            Object::ReturnValue(obj) => String::from(format!("Return value: {}", obj)),
            Object::Function { .. } => String::from("Function"),
            Object::BuiltinFunction(..) => String::from("Builtin Function"),
        };
        write!(f, "{}", repr)?;
        Ok(())
    }
}
