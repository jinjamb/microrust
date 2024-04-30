use std::fmt;

use crate::memory::Address;

#[derive(Debug, Clone, PartialEq)]
pub enum ParsedValue {
    Integer(isize),
    Boolean(bool),
    Unit,
    Pointer(Address),
}

impl fmt::Display for ParsedValue {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsedValue::Integer(i) => write!(f, "{}", i),
            ParsedValue::Boolean(b) => write!(f, "{}", b),
            ParsedValue::Unit => write!(f, "()"),
            ParsedValue::Pointer(a) => write!(f, "{:?}", a),
        }
    }
}