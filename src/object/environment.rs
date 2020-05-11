use crate::object::Object;
use core::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
    map: HashMap<String, Rc<Object>>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            map: HashMap::<String, Rc<Object>>::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: Rc<RefCell<Environment>>) -> Environment {
        let mut env = Environment::new();
        env.outer = Some(outer);
        env
    }

    pub fn get(&self, name: &str) -> Option<Rc<Object>> {
        let inner: Option<Rc<Object>> = self.map.get(name).map(|obj| Rc::clone(obj));

        let outer: Option<Rc<Object>> = self
            .outer
            .as_ref()
            .map(|env| env.borrow().get(name))
            .flatten();

        inner.or(outer)
    }

    pub fn set(&mut self, name: &str, obj: Rc<Object>) {
        self.map.insert(String::from(name), obj);
    }
}
