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

impl Token {
    pub fn get_precedence(&self) -> usize {
        match self.t {
            Type::Equals   => 1,
            Type::Plus     => 2,
            Type::Minus    => 2,
            Type::Multiply => 3,
            Type::Divide   => 3,
            
            _ => 0,
        }
    }
}