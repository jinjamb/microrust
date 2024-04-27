use crate::error::EvalError;
use crate::expression::Expression;
use crate::binop::Binop;
use crate::{instruction::Instruction, namespace::NameSpace};
use crate::identifier::Identifier;

impl Expression {

    pub fn eval(&self, namespace: &NameSpace) -> Result<isize, EvalError> {
        match self {
            Expression::Const(value) => Ok(*value),
            Expression::Identifier(id) => {
                Ok(namespace.find(id)?)
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
                            Err(EvalError::DivisionByZero(self.clone()))
                        } else {
                            Ok(left_val / right_val)
                        }
                    }
                    Binop::Mod => {
                        if right_val == 0 {
                            Err(EvalError::DivisionByZero(self.clone()))
                        } else {
                            Ok(left_val % right_val)
                        }
                    }
                }
            }
        }
    }
}

impl Instruction {

    pub fn exec(&mut self, ns: &mut NameSpace) -> Result<(Option<Identifier>, isize), EvalError>{

        match self {
            Instruction::Expr(expr) => {
                let entier = expr.eval(ns)?;
                Ok((None, entier))
            }
            Instruction::Let { id, expr } => {
                let entier = expr.eval(&ns)?;
                let _def = ns.declare(&Identifier::from(id.as_str()), entier);
                Ok((Some(Identifier::from(id.as_str())), entier))
            }
            Instruction::Block(vec) => {
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