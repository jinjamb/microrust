use crate::error::{EvalError, Error};
use crate::expression::Expression;
use crate::binop::Binop;
use crate::{instruction::Instruction, namespace::NameSpace};

impl Expression {

    pub fn eval(&self, namespace: &NameSpace) -> Result<isize, Error> {
        match self {
            Expression::Const(value) => Ok(*value),
            Expression::Identifier(id) => {
                match namespace.find(id) {
                    Ok(value) => Ok(value),
                    Err(err) => Err(err.into()),
                }
            },
            Expression::BinOp(left, op, right) => {
                let left_val = left.eval(&namespace)?;
                let right_val = right.eval(&namespace)?;
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

impl NameSpace {

    pub fn exec(&mut self, instr: &Instruction, ns: &mut NameSpace) -> Result<(), EvalError>{

        match instr {
            Instruction::Expr(expr) => {
                let _ = expr.eval(ns);
                Ok(())
            }
            Instruction::Let { id, expr } => {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifier::Identifier;
    use crate::expression::Expression;
    use crate::parser::Parse;

    #[test]
    fn test_eval_with_namespace() {
        let mut namespace = NameSpace::new();
        namespace.declare(&Identifier::from("x"), 1).unwrap();

        let expression = Expression::parse("1 + x").unwrap();
        match expression.eval(&namespace) {
            Ok(value) => {
                println!("Result: {}", value);
            }
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }

    #[test]
    fn test_eval_with_empty_namespace() {
        let namespace = NameSpace::new();

        let expression = Expression::parse("1 + x").unwrap();
        assert!(expression.eval(&namespace).is_err());
    }
}