use std::fmt::{self, Display};

use crate::expression::Expression;
use crate::identifier::Identifier;
use crate::r#type::Type;

use crate::parser::ParseError;

#[derive(Debug)]
pub enum Error {
    ParseError(ParseError),
    EvalError(EvalError),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum EvalError {
    DivisionByZero(Expression),
    Undefined(Identifier),
    AlreadyDefined(Identifier),
    NotMutable(Option<Expression>),
    TypeMismatch{expression: Expression, expected: Type, found: Type},
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::ParseError(e)
    }
}

impl From<EvalError> for Error {
    fn from(e: EvalError) -> Self {
        Error::EvalError(e)
    }
}

impl Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use EvalError::*;
        match self {
            DivisionByZero(e) => write!(f, "Division by zero, `{}` evaluates to 0", e),
            Undefined(id) => write!(f, "Undefined identifier: `{}`", id),
            AlreadyDefined(id) => write!(f, "Identifier `{}` already defined.", id),
            NotMutable(e) => write!(f, "Cell {}is not mutable.", e.as_ref().map(|e| format!("at `{}` ", e)).unwrap_or("".to_string())),
            TypeMismatch => write!(f, "Type mismatch"),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            ParseError(e) => write!(f, "Parse Error: {}", e),
            EvalError(e) => write!(f, "Evaluation Error: {}", e),
        }
    }
}