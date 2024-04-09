use crate::{Value, EvaluationError};

struct MemoryCell {
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

    pub fn get_value(&self) -> Result<&Value, EvaluationError>{
        if !self.mutable {
            return Err(EvaluationError::NotMutable);
        }
        if let Value::None = self.value {
            return Err(EvaluationError::NonAllocatedCell);
        }
        Ok(&self.value)
    }

    pub fn set_value(&mut self, value: Value) -> Result<(), EvaluationError> {
        if !self.mutable {
            return Err(EvaluationError::NotMutable);
        }
        self.value = value;
        Ok(())
    }
}