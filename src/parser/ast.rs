#[allow(dead_code)]
#[derive(Debug)]
#[derive(Clone)]
pub enum Node {
    Number(f64),
    Identifier(String),
    Infix { left: Box<Node>, right: Box<Node>, op: String },
}