use ::parser::operator;

// Any AST node. No distinguishing needed between
// expressions and statements, because everything
// is an expression.
pub trait Node {
    fn to_string(&self) -> String {
        String::from("<to_string not defined>")
    }
}

// A number literal, i.e. '420'
#[allow(unused)]
pub struct NumberLiteral {
    pub value: f64,
}

impl Node for NumberLiteral {
    fn to_string(&self) -> String {
        format!("({})", self.value)
    }
}

// An identifier, i.e. 'π'
#[allow(unused)]
pub struct Identifier {
    pub value: String,
}

impl Node for Identifier {
    fn to_string(&self) -> String {
        format!("({})", self.value)
    }
}

// An infix expression, i.e. 'ƒ + 3'
#[allow(unused)]
pub struct Infix {
    pub left: Box<Node>,
    pub right: Box<Node>,
    pub op: operator::Operator,
}

impl Node for Infix {
    fn to_string(&self) -> String {
        format!("({} {} {})", self.left.to_string(), self.op, self.right.to_string())
    }
}