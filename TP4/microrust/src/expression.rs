use crate::identifier::Identifier;
use crate::parser::ParseError;
use crate::binop::Binop;
use crate::parsing::leftexpression::LeftExpression;

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug)]
#[derive(Clone)]
pub enum Expression {
    Const(isize),
    Identifier(Identifier),
    BinOp(Box<Expression>, Binop, Box<Expression>),
    // à compléter ensuite
}
////////////////////////////////////////////////////////////////////////////////



use std::fmt::Display;

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Expression::*;
        match self {
            Const(i) => write!(f, "{}", i),
            Identifier(i) => write!(f, "{}", i),
            BinOp(lhs, op, rhs) => write!(f, "({} {} {})", lhs, op, rhs),
        }
    }
}


use crate::parsing::expression::Expression as ParseExpression;
use crate::parsing::parsedvalue::ParsedValue;

impl From<ParseExpression> for Result<Expression, ParseError> {
    fn from(expr: ParseExpression) -> Self {
        match expr {
            ParseExpression::Const(ParsedValue::Integer(i)) => 
                Ok(Expression::Const(i)),
            ParseExpression::ValueAt(LeftExpression::Identifier(id)) => 
                Ok(Expression::Identifier(id.clone())),
            ParseExpression::BinOp(lhs, binop, rhs) => {
                let lhs = Box::new(Self::from(*lhs)?);
                let rhs = Box::new(Self::from(*rhs)?);
                let binop: Result<Binop, ParseError> = <_>::from(binop);
                Ok(Expression::BinOp(lhs, binop?, rhs))
            },
            _ => { Err(ParseError::SyntaxNotSupported) }            
        }
    }
}

use crate::parser::Parse;

impl Parse for Expression {
    fn parse(input: &str) -> Result<Self, ParseError> {
        <_>::from(ParseExpression::parse(input)?)
    }
}