use crate::{
    error::{EvalError, Error},
    expression::{Expression, ParseExpression},
    binop::Binop,
    namespace::NameSpace,
    parsing::parsedvalue::ParsedValue,
};

impl From<ParseExpression> for Result<Expression, ParseError> {
    fn from(expr: ParseExpression) -> Self {
        match expr {
            ParseExpression::Const(ParsedValue::Integer(i)) => Ok(Expression::Const(i)),
            ParseExpression::ValueAt(LeftExpression::Identifier(id)) => {
                Ok(Expression::Identifier(id.clone()))
            }
            ParseExpression::BinOp(lhs, binop, rhs) => {
                let lhs = Box::new(Self::from(*lhs)?);
                let rhs = Box::new(Self::from(*rhs)?);
                let binop: Result<Binop, ParseError> = <_>::from(binop);
                Ok(Expression::BinOp(lhs, binop?, rhs))
            }
            _ => Err(ParseError::SyntaxNotSupported),
        }
    }
}