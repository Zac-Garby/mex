mod ast;

use super::scanner;
use super::scanner::token;
use self::ast::Node::*;
use std::num::ParseFloatError;
use std::result;

#[derive(Clone)]
#[derive(Debug)]
pub enum Error {
    UnexpectedEOF,
    WrongToken { expected: token::Type, got: token::Type },
    NoPrefix,
    NoInfix,
    InvalidFloat(ParseFloatError),
}

type Result<T> = result::Result<T, Error>;

pub struct Parser {
    scan: scanner::Scanner,
    cur: Option<token::Token>,
    peek: Option<token::Token>,
}

impl Parser {
    pub fn new(s: String) -> Parser {
        let scan = scanner::Scanner::new(s);

        let mut p = Parser{
            scan,
            cur: None,
            peek: None,
        };

        p.next();
        p.next();

        p
    }

    fn peek_precedence(&self) -> usize {
        let peek = self.peek.clone();

        match peek {
            Some(t) => t.get_precedence(),
            None => 0,
        }
    }

    fn cur_precedence(&self) -> usize {
        let cur = self.cur.clone();

        match cur {
            Some(t) => t.get_precedence(),
            None => 0,
        }
    }

    fn next(&mut self) {
        self.cur = self.peek.clone();
        self.peek = self.scan.next();
    }

    pub fn parse(&mut self) -> Result<ast::Node> {
        self.parse_expression(0)
    }

    fn parse_expression(&mut self, precedence: usize) -> Result<ast::Node> {
        let mut left = self.parse_prefix()?;

        while self.peek.is_some() && precedence < self.peek_precedence() {
            match self.parse_infix(left.clone()) {
                Ok(expr) => left = expr,
                Err(Error::NoInfix) => return Ok(left),
                Err(e) => return Err(e),
            };
        }

        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<ast::Node> {
        if let Some(cur) = self.cur.clone() {
            match cur.t {
                token::Type::Identifier => self.parse_id(cur),
                token::Type::Number => self.parse_num(cur),

                _ => Err(Error::NoPrefix),
            }
        } else {
            Err(Error::UnexpectedEOF)
        }
    }

    fn parse_id(&mut self, cur: token::Token) -> Result<ast::Node> {
        Ok(Identifier(cur.literal))
    }

    fn parse_num(&mut self, cur: token::Token) -> Result<ast::Node> {
        match cur.literal.parse() {
            Ok(v) => Ok(Number(v)),
            Err(e) => Err(Error::InvalidFloat(e)),
        }
    }

    fn parse_infix(&mut self, left: ast::Node) -> Result<ast::Node> {
        self.next();

        if let Some(cur) = self.cur.clone() {
            match cur.t {
                token::Type::Plus |
                token::Type::Minus |
                token::Type::Multiply |
                token::Type::Divide => self.parse_infix_op(cur, left),

                _ => Err(Error::NoInfix),
            }
        } else {
            Err(Error::UnexpectedEOF)
        }
    }

    fn parse_infix_op(&mut self, cur: token::Token, left: ast::Node) -> Result<ast::Node> {
        let op = cur.literal;
        let precedence = self.cur_precedence();
        self.next();
        let right = self.parse_expression(precedence)?;

        Ok(ast::Node::Infix {
            left: Box::new(left),
            right: Box::new(right),
            op
        })
    }
}
