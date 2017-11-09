mod ast;

use super::scanner;
use super::scanner::token;
use self::ast::Node::*;
use self::ast::Operator;
use std::num::ParseFloatError;

pub struct ParseError {
    pub message: String,
}

#[allow(unused)]
pub struct Parser {
    scan: scanner::Scanner,
    errors: Vec<ParseError>,
    cur: token::Token,
    peek: token::Token,
}

impl Parser {
    pub fn new(s: String) -> Parser {
        let scan = scanner::Scanner::new(s);

        let mut p = Parser{
            scan,
            errors: Vec::new(),
            cur: token::Token{literal: String::from(""), t: token::Type::Illegal},
            peek: token::Token{literal: String::from(""), t: token::Type::Illegal},
        };

        p.next();
        p.next();

        p
    }

    fn peek_precedence(&self) -> usize {
        self.peek.get_precedence()
    }

    fn cur_precedence(&self) -> usize {
        self.cur.get_precedence()
    }

    fn next(&mut self) {
        let peek = self.peek.clone();
        self.cur = peek;

        if let Some(next) = self.scan.next() {
            self.peek = next;
        } else {
            self.err("unexpected EOF")
        }

        if self.peek.t == token::Type::Illegal {
            let literal = self.peek.literal.clone();
            self.err(&format!("unexpected illegal token: {:?}", literal));
        }
    }

    fn err(&mut self, message: &str) {
        let err = ParseError{message: String::from(message)};
        self.errors.push(err);
    }

    pub fn parse_expression(&mut self, precedence: usize) -> Option<ast::Node> {
        let mut left = self.parse_prefix();

        if left.is_none() {
            let t = self.cur.clone().t;
            self.err(&format!("unexpected token: {:?}", t));
            return None
        }

        while precedence < self.peek_precedence() {
            let infix = self.parse_infix(left.clone().unwrap());

            if infix.is_none() {
                return left
            }

            self.next();
            left = infix;
        }

        left
    }

    fn parse_prefix(&mut self) -> Option<ast::Node> {
        match self.cur.t {
            token::Type::Identifier => Some(self.parse_id()),
            token::Type::Number     => Some(self.parse_number()),

            _ => None,
        }
    }

    fn parse_infix(&mut self, left: ast::Node) -> Option<ast::Node> {
        self.next();
    
        if self.cur.t == token::Type::Equals {
            self.parse_equals_infix(left)
        } else {
            self.parse_normal_infix(left)
        } 
    }

    fn parse_id(&mut self) -> ast::Node {
        Identifier(self.cur.literal.clone())
    }

    fn parse_number(&mut self) -> ast::Node {
        let val: Result<f64, ParseFloatError> = self.cur.literal.parse();

        match val {
            Ok(v) => Number(v),
            Err(e) => panic!(e),
        }
    }

    fn parse_equals_infix(&mut self, left: ast::Node) -> Option<ast::Node> {
        let precedence = self.cur_precedence();
        self.next();

        if let Some(right) = self.parse_expression(precedence) {
            Some(ast::Node::Infix{
                left: Box::new(left),
                op: Operator::Assign,
                right: Box::new(right),
            })
        } else {
            panic!("unexpected None from parse_expression")
        }
    }

    fn parse_normal_infix(&mut self, left: ast::Node) -> Option<ast::Node> {
        let op = match self.cur.t {
            token::Type::Plus => Operator::Add,
            token::Type::Minus => Operator::Subtract,
            token::Type::Multiply => Operator::Multiply,
            token::Type::Divide => Operator::Divide,

            _ => return None,
        };

        let precedence = self.cur_precedence();
        self.next();

        if let Some(right) = self.parse_expression(precedence) {
            Some(ast::Node::Infix{
                left: Box::new(left),
                op,
                right: Box::new(right),
            })
        } else {
            panic!("unexpected None from parse_expression")
        }
    }
}