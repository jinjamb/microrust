use crate::error::EvalError;
use crate::identifier::Identifier;
use crate::value::Value;
use crate::memorycell::MemoryCell;
use crate::memory::Address;
use std::collections::HashMap;

pub struct NameSpace(HashMap<Identifier, MemoryCell>);

impl NameSpace {

    pub fn new() -> Self {
        NameSpace(HashMap::new())
    }

    pub fn declare(&mut self, id: &Identifier, memcell: MemoryCell) -> Result<(), EvalError> {
//        self.0.try_insert(id, value).map_err(|_| EvalError::AlreadyDefined(id))
        if self.0.contains_key(&id) {
            Err(EvalError::AlreadyDefined(id.clone()))
        } else {
            self.0.insert(id.clone(), memcell);
            Ok(())
        }
    }

    pub fn find(&self, id: &Identifier) -> Result<Value, EvalError> {
        match self.0.get(id) {
            Some(MemoryCell) => MemoryCell.get_value(),
            None => Err(EvalError::Undefined(id.clone())),
        }
    }

    pub fn set(&mut self, id: &Identifier, value: Value) -> Result<(), EvalError> {
        match self.0.get_mut(id) {
            Some(MemoryCell) => {
                MemoryCell.set_value(value)
            },
            None => Err(EvalError::Undefined(id.clone())),
        }
    }

    pub fn contains_key(&self, id: &Identifier) -> bool {
        self.0.contains_key(id)
    }
}