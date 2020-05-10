use crate::object::Object;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
    map: HashMap<String, Rc<Object>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            map: HashMap::<String, Rc<Object>>::new(),
        }
    }

    pub fn get(&self, name: &str) -> Result<Rc<Object>, String> {
        self.map
            .get(name)
            .map(|obj| Rc::clone(obj))
            .ok_or(String::from(format!(
                "The identifier '{}' has not been bound",
                name
            )))
    }

    pub fn set(&mut self, name: &str, obj: Rc<Object>) {
        self.map.insert(String::from(name), obj);
    }
}
