mod scanner;
mod operator;

use scanner::token::Token;
use operator::Operator;

// Any AST node.
pub trait Node {
    fn to_string(&self) -> String {
        "<to_string not defined>"
    }
}

// An abstract syntax tree. Represents an entire
// program's syntax.
pub struct AST {
    pub statements: Vec<Node>,
}

impl Node for AST {
    fn to_string(&self) -> String {
        self.statements
            .iter()
            .fold(String::new(), |out, stmt| out + stmt.to_string());
    }
}

// A number literal, i.e. '420'
pub struct NumberLiteral {
    pub value: f64,
}

impl Node for NumberLiteral {
    fn to_string(&self) -> String {
        format!("{}", self.value)
    }
}

// An identifier, i.e. 'π'
pub struct Identifier {
    pub value: String,
}

impl Node for Identifier {
    fn to_string(&self) -> String {
        self.value
    }
}

// An infix expression, i.e. 'ƒ + 3'
pub struct Infix {
    pub left, right: Node,
    pub op: Operator,
}

impl Node for Infix {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.left.to_string(), self.op, self.right.to_string())
    }
}