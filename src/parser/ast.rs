mod scanner;

use scanner::token::Token;

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
