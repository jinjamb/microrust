use crate::error::{EvalError, Error};
use crate::expression::Expression;
use crate::binop::Binop;
use crate::namespace::NameSpace;

impl Expression {
    pub fn eval(&self, namespace: &NameSpace) -> Result<isize, Error> {
        match self.clone() {
            Expression::Const(value) => Ok(value),
            Expression::Identifier(id) => {
                match namespace.find(&id) {
                    Ok(value) => Ok(value),
                    Err(_err) => Err(EvalError::Undefined(id.clone()).into())
                }
            },
            Expression::BinOp(left, op, right) => {
                let left_val = left.eval(namespace)?;
                let right_val = right.eval(namespace)?;
                match op {
                    Binop::Add => Ok(left_val + right_val),
                    Binop::Sub => Ok(left_val - right_val),
                    Binop::Mul => Ok(left_val * right_val),
                    Binop::Div => {
                        if right_val == 0 {
                            Err(EvalError::DivisionByZero(self.clone()).into())
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                    Binop::Mod => {
                        if right_val == 0 {
                            Err(EvalError::DivisionByZero(self.clone()).into())
                        } else {
                            Ok(left_val % right_val)
                        }
                    }
                }
            }
        }
    }
}