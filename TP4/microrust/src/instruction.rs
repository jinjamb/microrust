use crate::identifier::Identifier;
use crate::expression::Expression;

#[derive(Debug)]
pub enum Instruction {
    Expr(Expression),
    Let{id:String, mutable: bool, expr:Expression},
    Block(Vec<Instruction>),
} 

use std::fmt::Display;
impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Instruction::*;
        match self {
            Expr(expr) => write!(f, "{}", expr),
            Let{id, mutable, expr} => {
                if *mutable {
                    write!(f, "let mut {} = {}", id, expr)
                } else {
                    write!(f, "let {} = {}", id, expr)
                }
            },
            Block(instrs) => { 
                write!(f, "{{{}}}", instrs.into_iter()
                                          .map(|x| x.to_string())
                                          .collect::<Vec<_>>()
                                          .join(";"))
            },
        }
    }
}


use crate::{parser::{Parse, ParseError}, parsing::instruction::Instruction as ParseInstruction};

impl From<ParseInstruction> for Result<Instruction, ParseError> {
    fn from(instr: ParseInstruction) -> Self {
        match instr {
            ParseInstruction::Expr(expr) => Ok(Instruction::Expr(Expression::parse(&expr.to_string())?)),
            ParseInstruction::Let{id, mutable, expr, ..} => Ok(Instruction::Let{id: id.to_string(), mutable, expr: Expression::parse(&expr.to_string())?}),
            ParseInstruction::Block(instrs) => {
                let instrs: Result<Vec<Instruction>, ParseError> = instrs.into_iter().map(|x| <_>::from(x)).collect();
                Ok(Instruction::Block(instrs?))
            },
            _ => { Err(ParseError::SyntaxNotSupported) }
        }
    }
}

impl Parse for Instruction {
    fn parse(input: &str) -> Result<Self, ParseError> {
        <_>::from(ParseInstruction::parse(input)?)
    }

}
