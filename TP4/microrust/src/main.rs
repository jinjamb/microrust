mod parsing;
mod parser;
mod error;
mod identifier;
mod binop;
mod expression;
mod eval;
mod namespace;
mod instruction;
mod namespacestack;
mod value;
mod r#type;
#[cfg(test)]
mod test_namespace;
mod test_namespace_stack;
mod memorycell;

use std::io::{self, BufRead, Write};
use namespacestack::NameSpaceStack;
use parser::Parse;
use instruction::Instruction;
use crate::namespace::NameSpace;
use crate::error::Error;
use crate::identifier::Identifier;
use crate::value::Value;
use crate::r#type::Type;

fn prompt() {
    print!("µRust # ");
    io::stdout().flush().unwrap();
}

fn parse_exec(input: &str, nss: &mut NameSpaceStack) -> Result<(Option<Identifier>, Value), Error> {
    match Instruction::parse(input) {
        Ok(mut instr) => {
            instr.exec(nss).map_err(|err| Error::EvalError(err))
        }
        Err(e) => Err(Error::ParseError(e)),
    }
}

fn main(){
    prompt();
    let mut nss = NameSpaceStack::new();
    nss.push(NameSpace::new());
    let stdin = io::stdin().lock();
    for line in stdin.lines() {
        let line = line.unwrap();
        match parse_exec(&line, &mut nss) {
            Ok((id, val)) => {
                println!("{} : {} = {}", id.unwrap_or(Identifier::from("-")), Type::from(&val), val);
            }
            Err(e) => {
                println!("{}", e);
            }
        }
        prompt();
    }
}

/*
fn main() {

    let mut nss = NameSpaceStack::new();
    nss.push(NameSpace::new());

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

            match i.exec(&mut nss) {
                Ok((option, entier)) => println!("{:?} : isize = {}", option, entier),
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            };

            break;
        }
    }
}*/