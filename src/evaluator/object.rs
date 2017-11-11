use std::fmt;

#[derive(Debug)]
#[derive(Clone)]
pub enum Object {
    Number(f64),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Object::Number(val) => write!(f, "{}", val),
        }
    }
}