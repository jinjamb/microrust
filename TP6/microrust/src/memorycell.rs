use crate::{error::EvalError, value::Value};

#[derive(Debug)]

pub struct MemoryCell {
    mutable: bool,
    value:  Value,
}

impl MemoryCell {
    pub fn new(mutable: bool, value: Value) -> MemoryCell {
        MemoryCell {
            mutable,
            value,
        }
    }

    pub fn is_mutable(&self) -> bool {
        self.mutable
    }

    pub fn get_value(&self) -> Result<Value, EvalError>{
        Ok(self.value.clone())
    }

    pub fn set_value(&mut self, value: Value) -> Result<(), EvalError> {
        if (&self).is_mutable() {Ok(self.value = value)}
        else {Err(EvalError::NotMutable(None))}
    }
}