use crate::parsing::expression::Expression;
use crate::parsing::leftexpression::LeftExpression;
use crate::identifier::Identifier;

#[derive(Debug)]
pub enum Instruction {
    Expr(Expression),
    Let{id:Identifier, mutable:bool, expr:Expression},
    Block(Vec<Instruction>),
    IfElse{
        cond: Expression,
        cond_true: Box<Instruction>,
        cond_false: Box<Instruction>,
    },
    WriteAt(LeftExpression, Expression),
    While(Expression, Box<Instruction>),
    Free(LeftExpression),
    Drop(LeftExpression)
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
                write!(f, "{{{}}}", instrs.into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(";"))
            },
            IfElse{cond, cond_true, cond_false} => {
                write!(f, "if {} {} else {}", cond, cond_true, cond_false)
            },
            WriteAt(lexpr, expr) => {
                write!(f, "{} = {}", lexpr, expr)
            },
            While(cond, instr) => {
                write!(f, "while {} {}", cond, instr)
            },
            Free(lexpr) => {
                write!(f, "free {}", lexpr)
            },
            Drop(lexpr) => {
                write!(f, "drop {}", lexpr)
            }
        }
    }
}


use pest::Parser;
use crate::parser::{ParseError, Parse};
use super::utils::{PestParser, Rule, parse_instr};

impl Parse for Instruction {
    fn parse(input: &str) -> Result<Self, ParseError> {
        match PestParser::parse(Rule::start_rule_instr, &input) {
            Ok(mut pairs) => {
                let first_rule = pairs.next().unwrap();
                match first_rule.as_rule() {
                    Rule::instr => {
                        parse_instr(&mut first_rule.into_inner())
                    }
                    _ => { panic!("the grammar is not as expected") }
                }                
            },
            Err(_e) => { Err(ParseError::CannotParse) }
        }
    }

}
