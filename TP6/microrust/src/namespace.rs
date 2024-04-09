use std::collections::HashMap;
use crate::memorycell::MemoryCell;
use crate::{error::EvalError, identifier::Identifier, parsing::expression::Expression, value::Value};

#[derive(Debug)]
pub struct NameSpace(HashMap<Identifier, MemoryCell>);



impl NameSpace {
    pub fn new() -> Self {
        NameSpace(HashMap::new())
    }

    pub fn declare(&mut self, id: &Identifier, mutable: bool, value: Value) -> Result<(), EvalError> {
//        self.0.try_insert(id, value).map_err(|_| EvalError::AlreadyDefined(id))
        if self.0.contains_key(&id) {
            Err(EvalError::AlreadyDefined(id.clone()))git pu
        } else {
            self.0.insert(id.clone(), (mutable, value));
            Ok(())
        }
    }

    pub fn find(&self, id: &Identifier) -> Result<Value, EvalError> {
        match self.0.get(id) {
            Some((_, v)) => Ok(v.clone()),
            None => Err(EvalError::Undefined(id.clone())),
        }
    }

    pub fn set(&mut self, id: &Identifier, value: Value) -> Result<(), EvalError> {
        match self.0.get_mut(id) {
            Some((true, v)) => {
                *v = value;
                Ok(())
            },
            Some((false, _)) => Err(EvalError::NotMutable(Some(Expression::Identifier(id.clone())))),
            None => Err(EvalError::Undefined(id.clone())),
        }
    }
}


/* 
#[cfg(test)]
mod test_namespace {

    use super::*;

    #[test]
    fn test_declare() {
        let mut ns = NameSpace::new();
        let id = Identifier::from("x");
        assert!(ns.declare(&id, false, 42).is_ok());
        match ns.declare(&id, false, 42) {
            Ok(_) => panic!("Identifier should not be declared twice"),
            Err(EvalError::AlreadyDefined(id2)) => assert_eq!(id, id2),
            Err(_) => panic!("Unexpected error"),
        }
    }

    #[test]
    fn test_find() {
        let mut ns = NameSpace::new();
        let id = Identifier::from("x");
        match ns.find(&id) {
            Ok(_) => panic!("Identifier should not be found"),
            Err(EvalError::Undefined(id2)) => assert_eq!(id, id2),
            Err(_) => panic!("Unexpected error"),
        }
        assert!(ns.declare(&id, false, 42).is_ok());
        match ns.find(&id) {
            Ok(42) => (),
            _ => panic!("Identifier should be found"),
        }
    }

}

*/
