use crate::object::Object;
use core::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment<'a> {
    map: HashMap<String, Rc<Object<'a>>>,
    outer: Option<Rc<RefCell<Environment<'a>>>>,
}

impl<'a> Environment<'a> {
    pub fn new() -> Environment<'a> {
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

    pub fn get(&self, name: &str) -> Option<Rc<Object<'a>>> {
        let inner: Option<Rc<Object>> = self.map.get(name).map(|obj| Rc::clone(obj));

        let outer: Option<Rc<Object>> = self
            .outer
            .as_ref()
            .map(|env| env.borrow().get(name))
            .flatten();

        inner.or(outer)
    }

    pub fn set(&mut self, name: &str, obj: Rc<Object<'a>>) {
        self.map.insert(String::from(name), obj);
    }
}
