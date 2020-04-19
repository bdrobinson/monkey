use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
    ReturnValue(Box<Object>),
}
impl Object {
    pub fn type_name(&self) -> String {
        let string = match self {
            Object::Integer(_) => "Integer",
            Object::Boolean(_) => "Boolean",
            Object::Null => "Null",
            Object::ReturnValue(_) => "Return value",
        };
        String::from(string)
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let repr: String = match self {
            Object::Integer(value) => value.to_string(),
            Object::Boolean(value) => value.to_string(),
            Object::Null => String::from("null"),
            Object::ReturnValue(obj) => String::from(format!("Return value: {}", obj)),
        };
        write!(f, "{}", repr)?;
        Ok(())
    }
}
