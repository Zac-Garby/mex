// The various types of operators.
#[allow(unused)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[allow(dead_code)]
pub enum Node {
    Number(f64),
    Identifier(String),
    Infix { left: Box<Node>, right: Box<Node>, op: Operator },
}