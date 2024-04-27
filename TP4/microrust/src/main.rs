mod parsing;
mod parser;
mod error;
mod identifier;
mod binop;
mod expression;
mod eval;
mod namespace;
mod instruction;
#[cfg(test)]
mod test_namespace;

use std::io::{self, BufRead, Write};
use parser::Parse;
use namespace::NameSpace;
use instruction::Instruction;

fn prompt() {
    print!("µRust # ");
    io::stdout().flush().unwrap();
}

fn main() {

    let mut namespace = NameSpace::new();

    loop {


        prompt();

        for line in io::stdin().lock().lines() {
            let input = match line {
                Ok(input) => input,
                Err(_) => {
                    println!("Error: Unable to read input");
                    continue;
                }
            };

            let mut i = match Instruction::parse(&input) {
                Ok(expr) => expr,
                Err(_) => {
                    println!("Cannot parse");
                    continue;
                }
            };

            match i.exec(&mut namespace) {
                Ok((option, entier)) => println!("{:?} : isize = {}", option, entier),
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            };

            break;
        }
    }
}