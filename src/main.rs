extern crate calc;

mod scanner;

use std::io;
use std::io::prelude::*;

use scanner::*;

fn main() {
    loop {
        print!(">> ");
        io::stdout().flush().expect("couldn't flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("couldn't read input");

        let mut scan = Scanner::new(input);

        loop {
            if let Some(tok) = scan.next() {
                println!("token: {:?}", tok);
            } else {
                break
            }
        }
    }
}
