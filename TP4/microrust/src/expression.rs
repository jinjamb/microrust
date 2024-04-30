use crate::identifier::Identifier;
use crate::parser::ParseError;
use crate::binop::Binop;
use crate::parsing::leftexpression::LeftExpression;
use crate::parsing::parsedvalue::ParsedValue;

////////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone)]
pub enum Expression {
    Const(ParsedValue),
    Identifier(Identifier),
    BinOp(Box<Expression>, Binop, Box<Expression>),
    Conditional{
        cond: Box<Expression>,
        cond_true: Box<Expression>,
        cond_false: Box<Expression>,
    },
    AmpersAnd(Box<Expression>),
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
            Conditional { cond, cond_true, cond_false } => write!(f, "({}) ? {}  : {} ", cond, cond_true, cond_false),
            AmpersAnd(e) => write!(f, "&{}", e),
        }
    }
}


use crate::parsing::expression::Expression as ParseExpression;

impl From<ParseExpression> for Result<Expression, ParseError> {
    fn from(expr: ParseExpression) -> Self {
        match expr {
            ParseExpression::Const(v) => 
                Ok(Expression::Const(v)),
            ParseExpression::ValueAt(LeftExpression::Identifier(id)) => 
                Ok(Expression::Identifier(id.clone())),
            ParseExpression::BinOp(lhs, binop, rhs) => {
                let lhs = Box::new(Self::from(*lhs)?);
                let rhs = Box::new(Self::from(*rhs)?);
                let binop: Result<Binop, ParseError> = <_>::from(binop);
                Ok(Expression::BinOp(lhs, binop?, rhs))
            },
            ParseExpression::Conditional{ cond, cond_true, cond_false } => {
                Ok(Expression::Conditional { 
                    cond: Box::new(Self::from(*cond)?) , 
                    cond_true: Box::new(Self::from(*cond_true)?),
                    cond_false: Box::new(Self::from(*cond_false)?) 
                })
            }
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