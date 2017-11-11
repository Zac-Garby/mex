extern crate calc;

use std::io;
use std::io::prelude::*;

use calc::parser::*;

fn main() {
    loop {
        print!(">> ");
        io::stdout().flush().expect("couldn't flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("couldn't read input");

        let mut parse = Parser::new(input);
        let expr = parse.parse();
        println!("{:?}", expr);
    }
}
