extern crate mex;

use std::io;
use std::io::prelude::*;

use mex::parser;
use mex::evaluator;
use mex::evaluator::context;

fn main() {
    let mut context = context::Context::new();

    loop {
        print!(">> ");
        io::stdout().flush().expect("couldn't flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("couldn't read input");

        let mut parse = parser::Parser::new(input);

        match parse.parse() {
            Ok(node) => {
                match evaluator::eval_node_in(node, &mut context) {
                    Ok(result) => println!("{}", result),
                    Err(err) => println!("eval error: {:?}", err),
                }
            }

            Err(e) => println!("parse error: {:?}", e),
        };
    }
}
