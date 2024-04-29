use crate::error::EvalError;
use crate::expression::Expression;
use crate::binop::Binop;
use crate::namespacestack::NameSpaceStack;
use crate::{instruction::Instruction, namespace::NameSpace};
use crate::identifier::Identifier;
use crate::value::Value;
use crate::r#type::Type;


impl Expression {

    fn eval_and_cast_to_int(&self, nss: &mut NameSpaceStack) -> Result<isize, EvalError> {
        let v = self.eval(nss)?;
        v.to_int()
         .map_err(|_| EvalError::TypeMismatch{
            expression: self.clone(), 
            expected: Type::Int, 
            found: Type::from(&v)})
    }

    pub fn eval(&self, nss: &mut NameSpaceStack) -> Result<Value, EvalError> {
        match self {
            Expression::Const(value) => Ok(Value::from(*value)),
            Expression::Identifier(id) => {
                Ok(nss.find(id)?) 
            },
            Expression::BinOp(e1, Binop::Add, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                Ok(Value::Integer(v1 + v2))
            },
            Expression::BinOp(e1, Binop::Sub, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                Ok(Value::Integer(v1 - v2))
            },
            Expression::BinOp(e1, Binop::Mul, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                Ok(Value::Integer(v1 * v2))
            },
            Expression::BinOp(e1, Binop::Div, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                Ok(Value::Integer(v1 / v2))
            },
            Expression::BinOp(e1, Binop::Mod, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                Ok(Value::Integer(v1 % v2))
            },
        }
    }
}

impl Instruction {

    pub fn exec(&mut self, nss: &mut NameSpaceStack) -> Result<(Option<Identifier>, Value), EvalError>{

        match self {
            Instruction::Expr(expr) => {
                let entier = expr.eval(nss)?;
                Ok((None, entier))
            }
            Instruction::Let { id, mutable, expr } => {
                let entier = expr.eval(nss)?;
                let ismut = mutable;
                let _def = nss.declare(&Identifier::from(id.as_str()), *ismut, &entier);
                Ok((Some(Identifier::from(id.as_str())), entier.clone()))
            }
            Instruction::Block(vec) => {
                nss.push(NameSpace::new());
                let mut return_value = Value::Unit;
                for instr in vec {
                    let (_id, val) = instr.exec(nss).map_err(|err| {nss.pop(); err})?;
                    return_value = val;
                };
                nss.pop();
                Ok((None, return_value))
            }
        }
    }
}
/*
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
}*/

/*
Expression::BinOp(left, op, right) => {
                let left_val = left.eval(nss)?;
                let right_val = right.eval(nss)?;
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
 */