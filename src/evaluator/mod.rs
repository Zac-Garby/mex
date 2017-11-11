mod object;
mod context;

use super::parser::ast;
use std::result;

type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotImplemented,
}

pub fn eval_node_in(node: ast::Node, ctx: &mut context::Context) -> Result<object::Object> {
    match node {
        ast::Node::Number(val) => Ok(object::Object::Number(val)),

        _ => Err(Error::NotImplemented),
    }
}

pub fn eval_node(node: ast::Node) -> Result<object::Object> {
    let ref mut ctx = context::Context::new();
    eval_node_in(node, ctx)
}