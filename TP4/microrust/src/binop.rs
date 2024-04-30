////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Leq,
    Geq,
    Lt,
    Gt,
    Eq,
    Neq,
    And,
    Or
}
////////////////////////////////////////////////////////////////////////////////

use std::fmt::{self, Display};

impl Display for Binop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Binop::*;
        match self {
            Add => write!(f, "+"),
            Sub => write!(f, "-"),
            Mul => write!(f, "*"),
            Div => write!(f, "/"),
            Mod => write!(f, "%"),
            Leq => write!(f, "<="),
            Geq => write!(f, ">="),
            Lt => write!(f, "<"),
            Gt => write!(f, ">"),
            Eq => write!(f, "=="),
            Neq => write!(f, "!="),
            And => write!(f, "&&"),
            Or => write!(f, "||"),
        }
    }
}


use crate::parser::{Parse, ParseError};
use crate::parsing::binop::Binop as ParseBinop;

impl From<ParseBinop> for Result<Binop, ParseError> {
    fn from(binop: ParseBinop) -> Self {
        match binop {
            ParseBinop::Add => Ok(Binop::Add),
            ParseBinop::Sub => Ok(Binop::Sub),
            ParseBinop::Mul => Ok(Binop::Mul),
            ParseBinop::Div => Ok(Binop::Div),
            ParseBinop::Mod => Ok(Binop::Mod),
            ParseBinop::Leq => Ok(Binop::Leq),
            ParseBinop::Geq => Ok(Binop::Geq),
            ParseBinop::Lt => Ok(Binop::Lt),
            ParseBinop::Gt => Ok(Binop::Gt),
            ParseBinop::Eq => Ok(Binop::Eq),
            ParseBinop::Neq => Ok(Binop::Neq),
            ParseBinop::And => Ok(Binop::And),
            ParseBinop::Or => Ok(Binop::Or),
        }
    }
}

impl Parse for Binop {
    fn parse(input: &str) -> Result<Self, ParseError> {
        <_>::from(ParseBinop::parse(input)?)
    }
}