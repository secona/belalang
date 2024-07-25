use crate::object::Object;
use crate::error::RuntimeError;

#[derive(Default, Debug)]
pub struct Stack {
    stack: Vec<Object>,
    popped: Object,
    pointer: usize,
}

impl Stack {
    pub fn top(&mut self) -> Result<&Object, RuntimeError> {
        if self.pointer == 0 {
            return Err(RuntimeError::StackUnderflow);
        }

        self.stack
            .get(self.pointer - 1)
            .ok_or(RuntimeError::StackUnderflow)
    }

    pub fn take_last(&mut self) -> Object {
        std::mem::take(&mut self.popped)
    }

    pub fn pop(&mut self) -> Result<(), RuntimeError> {
        self.popped = self.pop_take()?;

        Ok(())
    }

    pub fn pop_take(&mut self) -> Result<Object, RuntimeError> {
        if self.pointer == 0 {
            return Err(RuntimeError::StackUnderflow);
        }

        self.pointer -= 1;

        Ok(self.stack.remove(self.pointer))
    }

    pub fn push(&mut self, object: Object) -> Result<(), RuntimeError> {
        self.stack.push(object);
        self.pointer += 1;

        Ok(())
    }
}
