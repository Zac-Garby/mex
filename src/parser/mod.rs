mod ast;

use super::scanner;
use super::scanner::token;
use self::ast::Node::*;

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

    pub fn parse_expression(&self) -> ast::Node {
        self.parse_prefix()
    }

    fn parse_prefix(&self) -> ast::Node {
        Number(0.0)
    }

    fn parse_infix(&self, left: ast::Node) -> ast::Node {
        left
    }

    fn parse_id(&self) -> ast::Node {
        Identifier(String::from("hello"))
    }

    fn parse_number(&self) -> ast::Node {
        Number(0.0)
    }
}