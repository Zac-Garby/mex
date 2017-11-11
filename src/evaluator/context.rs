use super::object;
use std::collections::HashMap;

pub struct Context <'a> {
    pub store: Store,
    pub outer: Option<&'a mut Context<'a>>,
}

impl <'a> Context <'a> {
    pub fn new() -> Context<'a> {
        let store = HashMap::new();

        Context{
            store: store,
            outer: None,
        }
    }

    /* pub fn inside(outer: &'a mut Context<'a>) -> Context<'a> {
        let store = HashMap::new();

        Context{
            store: &mut store,
            outer: Some(outer),
        }
    } */

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

    pub fn get<'b>(&'b self, name: String) -> Option<&'b object::Object> {
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