use crate::error::EvalError;
use crate::expression::Expression;
use crate::binop::Binop;
use crate::memory::Memory;
use crate::namespacestack::NameSpaceStack;
use crate::parsing::parsedvalue::ParsedValue;
use crate::{instruction::Instruction, namespace::NameSpace};
use crate::identifier::Identifier;
use crate::value::Value;
use crate::memory::Address;
use crate::r#type::Type;


impl Expression {

    fn eval_and_cast_to_int(&self, nss: &mut NameSpaceStack) -> Result<isize, EvalError> {
        let v = self.eval(nss)?;
        let cloned_v = v.clone();
        cloned_v.to_int()
            .map_err(|_| EvalError::TypeMismatch{
                expression: self.clone(), 
                expected: Type::Int, 
                found: Type::from(&v)})
    }

    fn eval_and_cast_to_bool(&self, nss: &mut NameSpaceStack) -> Result<bool, EvalError> {
        let v = self.eval(nss)?;
        let cloned_v = v.clone();
        cloned_v.to_bool()
            .map_err(|_| EvalError::TypeMismatch{
                expression: self.clone(), 
                expected: Type::Bool, 
                found: Type::from(&v)})
    }

    fn eval_to_address(&self, nss: &mut NameSpaceStack) -> Result<Address, EvalError> {
        match self {
            Expression::Identifier(id) => {
                nss.get_address(id)
            }
            _ => {
                Err(EvalError::TypeMismatch {
                    expression: self.clone(),
                    expected: Type::from(&Value::from(ParsedValue::Unit)),
                    found: Type::Unit,
                })
            }
        }
    }

    pub fn eval(&self, nss: &mut NameSpaceStack) -> Result<Value, EvalError> {
        match self {
            Expression::Const(value) => Ok(Value::from(value.clone())),
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
            Expression::BinOp(e1, Binop::Leq, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                let mut boolean = false;
                if v1 <= v2 {
                    boolean = true;
                    Ok(Value::Boolean(boolean))
                }
                else {
                    Ok(Value::Boolean(boolean))
                }
            },
            Expression::BinOp(e1, Binop::Geq, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                let mut boolean = false;
                if v1 <= v2 {
                    Ok(Value::Boolean(boolean))
                }
                else {
                    boolean = true;
                    Ok(Value::Boolean(boolean))
                }
            },
            Expression::BinOp(e1, Binop::Lt, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                let mut boolean = false;
                if v1 <= v2 {
                    boolean = true;
                    Ok(Value::Boolean(boolean))
                }
                else {
                    Ok(Value::Boolean(boolean))
                }
            },
            Expression::BinOp(e1, Binop::Gt, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                let mut boolean = false;
                if v1 < v2 {
                    Ok(Value::Boolean(boolean))
                }
                else {
                    boolean = true;
                    Ok(Value::Boolean(boolean))
                }
            },
            Expression::BinOp(e1, Binop::Eq, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                let mut boolean = false;
                if v1 == v2 {
                    boolean = true;
                    Ok(Value::Boolean(boolean))
                }
                else {
                    Ok(Value::Boolean(boolean))
                }
            },
            Expression::BinOp(e1, Binop::Neq, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                let mut boolean = false;
                if v1 != v2 {
                    boolean = true;
                    Ok(Value::Boolean(boolean))
                }
                else {
                    Ok(Value::Boolean(boolean))
                }
            },
            Expression::BinOp(e1, Binop::And, e2) => {
                let v1 = e1.eval_and_cast_to_bool(nss)?;
                let v2 = e2.eval_and_cast_to_bool(nss)?;
                let mut boolean = false;
                if (v1 == true) && (v2==true) {
                    boolean = true;
                    Ok(Value::Boolean(boolean))
                }
                else {
                    Ok(Value::Boolean(boolean))
                }
            },
            Expression::BinOp(e1, Binop::Or, e2) => {
                let v1 = e1.eval_and_cast_to_bool(nss)?;
                let v2 = e2.eval_and_cast_to_bool(nss)?;
                let mut boolean = false;
                if (v1 == true) || (v2==true) {
                    boolean = true;
                    Ok(Value::Boolean(boolean))
                }
                else {
                    Ok(Value::Boolean(boolean))
                }
            },
            Expression::Conditional { cond, cond_true, cond_false } => {
                let cond_val = cond.eval_and_cast_to_bool(nss)?;
                if cond_val {
                    cond_true.eval(nss)
                } else {
                    cond_false.eval(nss)
                }
            }
            Expression::AmpersAnd(e) => {
                if let Expression::Identifier(id) = &**e {
                    if let Ok(address) = nss.get_address(id) {
                        Ok(Value::Pointer(address))
                    } else {
                        Err(EvalError::Undefined(id.clone()))
                    }
                } else {
                    Err(EvalError::TypeMismatch {
                        expression: *e.clone(),
                        expected: Type::Pointer,
                        found: Type::Unit,
                    })
                }
            }
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
                let id = &Identifier::from(id.as_str());
                let _def = nss.declare(id, *ismut, &entier);
                Ok((Some(id.clone()), entier.clone()))
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
Expression::BinOp(e1, Binop::Leq, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                if v1 <= v2 {
                    true
                }
                else {
                    false
                }
            },
            Expression::BinOp(e1, Binop::Geq, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                if v1 >= v2 {
                    true
                }
                else {
                    false
                }
            },
            Expression::BinOp(e1, Binop::Lt, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                if v1 < v2 {
                    true
                }
                else {
                    false
                }
            },
            Expression::BinOp(e1, Binop::Gt, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                if v1 > v2 {
                    true
                }
                else {
                    false
                }
            },
            Expression::BinOp(e1, Binop::Eq, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                if v1 == v2 {
                    true
                }
                else {
                    false
                }
            },
            Expression::BinOp(e1, Binop::Neq, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                if v1 != v2 {
                    true
                }
                else {
                    false
                }
            },
            Expression::BinOp(e1, Binop::And, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                if (v1 == true) && (v2==true) {
                    true
                }
                else {
                    false
                }
            },
            Expression::BinOp(e1, Binop::Or, e2) => {
                let v1 = e1.eval_and_cast_to_int(nss)?;
                let v2 = e2.eval_and_cast_to_int(nss)?;
                if (v1 == true) || (v2==true) {
                    true
                }
                else {
                    false
                }
            },
 */
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