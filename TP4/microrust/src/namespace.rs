use crate::error::EvalError;
use crate::identifier::Identifier;
use std::collections::HashMap;

pub struct NameSpace (HashMap<Identifier, isize>);

impl NameSpace {

    pub fn new() -> Self {
        NameSpace(HashMap::new())
    }

    pub fn find(&self, id: &Identifier) -> Result<isize, EvalError> {
        match self.0.get(id) {
            Some(&value) => Ok(value),
            None => Err(EvalError::Undefined(id.clone())),
        }
    }

    pub fn declare(&mut self, id: &Identifier, val: isize) -> Result<(), EvalError> {
        if self.0.get(id).is_some() {
            Err(EvalError::AlreadyDefined(id.clone()))
        } else {
            self.0.insert(id.clone(), val);
            Ok(())
        }
    }
}