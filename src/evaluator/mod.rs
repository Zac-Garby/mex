mod object;
mod context;

use super::parser::ast;
use self::ast::Node;
use self::object::Object;
use std::result;

type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidOperands,
    InvalidOperator,
    NotDefined,
    NotImplemented,
}

pub fn eval_node_in(node: ast::Node, ctx: &mut context::Context) -> Result<object::Object> {
    match node {
        ast::Node::Number(val) => Ok(Object::Number(val)),

        ast::Node::Identifier(id) => {
            if let Some(val) = ctx.get(id) {
                Ok(val.clone())
            } else {
                Err(Error::NotDefined)
            }
        }

        ast::Node::Infix{left, right, op} => {
            if let Node::Identifier(id) = *left {
                let right = eval_node_in(*right, ctx)?;
                ctx.set(&id, right.clone());
                Ok(right)
            } else {
                let left = eval_node_in(*left, ctx)?;
                let right = eval_node_in(*right, ctx)?;

                match (left, right) {
                    (Object::Number(l), Object::Number(r)) => {
                        Ok(Object::Number(match op.as_ref() {
                            "+" => l + r,
                            "-" => l - r,
                            "*" => l * r,
                            "/" => l / r,

                            _ => return Err(Error::InvalidOperator),
                        }))
                    }

                    _ => Err(Error::InvalidOperands),
                }
            }            
        }

        _ => Err(Error::NotImplemented),
    }
}

pub fn eval_node(node: ast::Node) -> Result<object::Object> {
    let ref mut ctx = context::Context::new();
    eval_node_in(node, ctx)
}