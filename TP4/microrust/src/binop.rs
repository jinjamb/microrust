////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Binop {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    // à compléter ensuite
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
            _ => { Err(ParseError::SyntaxNotSupported) }
        }
    }
}

impl Parse for Binop {
    fn parse(input: &str) -> Result<Self, ParseError> {
        <_>::from(ParseBinop::parse(input)?)
    }
}