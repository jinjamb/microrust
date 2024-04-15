use std::fmt;


#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(isize),
    Boolean(bool),
    Unit,
}

impl fmt::Display for Value {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "{}", i),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Unit => write!(f, "()"),
        }
    }
}