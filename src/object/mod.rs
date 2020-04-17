use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let repr: String = match self {
            Object::Integer(value) => value.to_string(),
            Object::Boolean(value) => value.to_string(),
            Object::Null => String::from("null"),
        };
        write!(f, "{}", repr)?;
        Ok(())
    }
}
