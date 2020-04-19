use crate::object::Object;
use std::collections::HashMap;

pub struct Environment {
    map: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            map: HashMap::<String, Object>::new(),
        }
    }

    pub fn get(&self, name: &str) -> Result<&Object, String> {
        self.map.get(name).ok_or(String::from(format!(
            "The identifier '{}' has not been bound",
            name
        )))
    }

    pub fn set(&mut self, name: &str, obj: Object) {
        self.map.insert(String::from(name), obj);
    }
}
