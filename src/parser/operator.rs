use std::fmt;

// The various types of operators.
#[allow(unused)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            &Operator::Add      => "+",
            &Operator::Subtract => "-",
            &Operator::Multiply => "*",
            &Operator::Divide   => "/",
        })
    }
}