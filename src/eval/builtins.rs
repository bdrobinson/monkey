use crate::object::{BuiltinFunction, Object};
use std::convert::TryInto;
use std::rc::Rc;

#[derive(Debug)]
struct Len;
impl BuiltinFunction for Len {
    fn run<'a>(&self, arguments: &[Rc<Object<'a>>]) -> Result<Rc<Object<'a>>, String> {
        if arguments.len() != 1 {
            return Err(String::from("len takes exactly 1 argument"));
        }
        let arg: &Rc<Object> = arguments.get(0).unwrap();
        match arg.as_ref() {
            Object::String(string) => {
                Ok(Rc::new(Object::Integer(string.len().try_into().unwrap())))
            }
            _ => Err(String::from("Only strings can be passed to len")),
        }
    }
}

#[derive(Debug)]
struct Print;
impl BuiltinFunction for Print {
    fn run<'a>(&self, arguments: &[Rc<Object<'a>>]) -> Result<Rc<Object<'a>>, String> {
        if arguments.len() != 1 {
            return Err(String::from("print takes exactly 1 argument"));
        }
        let arg0 = arguments.get(0).unwrap();
        println!("{}", arg0);
        Ok(Rc::new(Object::Null))
    }
}

pub fn get_builtin_fn(name: &str) -> Option<Box<dyn BuiltinFunction>> {
    match name {
        "len" => Some(Box::new(Len)),
        "print" => Some(Box::new(Print)),
        _ => None,
    }
}
