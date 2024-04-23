// LISTE DES MODULES
mod parsing;
mod parser;
mod error;
mod identifier;
mod binop;
mod expression;
mod eval;
mod namespace;
mod test_namespace;
mod instruction;

// LISTE DES IMPORTS
use std::io::{self, BufRead, Write};
use expression::Expression;
use parser::Parse;
use namespace::NameSpace;


// AFFICHAGE DU PROMPT
fn prompt() {
    print!("µRust # ");
    io::stdout().flush().unwrap();
}


// FONCTION PRINCIPALE
fn main() {
    loop {

        let namespace = NameSpace::new();

        prompt();

        for line in io::stdin().lock().lines() {
            let input = match line {
                Ok(input) => input,
                Err(_) => {
                    println!("Error: Unable to read input");
                    continue;
                }
            };

            let e = match Expression::parse(&input) {
                Ok(expr) => expr,
                Err(_) => {
                    println!("Cannot parse");
                    continue;
                }
            };

            let n = match e.eval(&namespace) {
                Ok(value) => value,
                Err(err) => {
                    println!("Evaluation Error: {}", err);
                    continue;
                }
            };

            println!("- : isize = {}", n);

            break;
        }
    }
}