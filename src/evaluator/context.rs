use super::object;
use std::collections::HashMap;

pub struct Context <'a> {
    pub store: Store,
    pub outer: Option<&'a mut Context<'a>>,
}

impl <'a> Context <'a> {
    pub fn new() -> Context<'a> {
        Context{
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn inside(outer: &'a mut Context<'a>) -> Context<'a> {
        Context{
            store: HashMap::new(),
            outer: Some(outer),
        }
    }

    pub fn has(&self, name: &String) -> bool {
        self.store.contains_key(name)
    }

    pub fn set(&mut self, name: &String, value: object::Object) {
        if let Some(ref mut outer) = self.outer {
            if outer.has(name) {
                return outer.set(name, value);
            }
        }

        self.declare(name, value);
    }

    pub fn declare(&mut self, name: &String, value: object::Object) {
        self.store.insert(name.clone(), value);
    }

    pub fn get(&'a self, name: String) -> Option<&'a object::Object> {
        if self.store.contains_key(&name) {
            self.store.get(&name)
        } else {
            match self.outer {
                Some(ref outer) => outer.get(name),
                None => None,
            }
        }
    }
}

pub type Store = HashMap<String, object::Object>;