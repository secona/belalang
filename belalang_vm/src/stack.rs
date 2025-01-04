use crate::error::RuntimeError;
use crate::object::Object;

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

    pub fn pop_take_n(&mut self, n: usize) -> Result<Vec<Object>, RuntimeError> {
        (0..n)
            .map(|_| self.pop_take())
            .collect::<Result<Vec<_>, _>>()
    }

    pub fn push(&mut self, object: Object) -> Result<(), RuntimeError> {
        self.stack.push(object);
        self.pointer += 1;

        Ok(())
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }
}
