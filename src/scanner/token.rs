use std::string::String;

#[derive(Debug)]
#[derive(PartialEq)]
#[allow(dead_code)]
#[derive(Clone)]
#[derive(Copy)]
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

#[derive(Debug)]
pub struct Token {
    pub t: Type,
    pub literal: String,
}