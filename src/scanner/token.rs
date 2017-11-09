use std::cmp::Eq;

#[allow(dead_code)]
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
#[derive(Hash)]
pub enum Type {
    Illegal,

    // Literals
    Identifier,
    Number,

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,

    // Misc. Symbols
    LeftParen,
    RightParen,
}

impl Eq for Type {}

#[derive(Debug)]
#[derive(Clone)]
pub struct Token {
    pub t: Type,
    pub literal: String,
}