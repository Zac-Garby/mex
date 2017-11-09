mod ast;

use super::scanner;
use super::scanner::token;
use self::ast::Node::*;
use self::ast::Operator;

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

        let p = Parser{
            scan,
            errors: Vec::new(),
            cur: token::Token{literal: String::from(""), t: token::Type::Illegal},
            peek: token::Token{literal: String::from(""), t: token::Type::Illegal},
        };

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
        self.parse_prefix()
    }

    fn parse_prefix(&mut self) -> Option<ast::Node> {
        match self.cur.t {
            token::Type::Identifier => Some(self.parse_id()),
            token::Type::Number     => Some(self.parse_number()),

            _ => None,
        }
    }

    fn parse_infix(&mut self, left: ast::Node) -> ast::Node {
        left
    }

    fn parse_id(&mut self) -> ast::Node {
        Identifier(String::from("hello"))
    }

    fn parse_number(&mut self) -> ast::Node {
        Number(0.0)
    }

    fn parse_normal_infix(&mut self, left: ast::Node) -> ast::Node {
        let op = match self.cur.t {
            token::Type::Plus => Operator::Add,
            token::Type::Minus => Operator::Subtract,
            token::Type::Multiply => Operator::Multiply,
            token::Type::Divide => Operator::Divide,

            _ => panic!("invalid infix operator token type: {:?}", self.cur.t),
        };

        let precedence = self.cur_precedence();
        self.next();

        if let Some(right) = self.parse_expression(precedence) {
            ast::Node::Infix{
                left: Box::new(left),
                op,
                right: Box::new(right),
            }
        } else {
            panic!("unexpected None from parse_expression")
        }
    }
}