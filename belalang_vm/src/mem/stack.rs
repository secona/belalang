use crate::error::RuntimeError;

const STACK_SIZE: usize = 4096;

pub struct Stack {
    stack: [u8; STACK_SIZE],
    cap: usize,
    sp: usize,
    fp: usize,
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: [0; STACK_SIZE],
            cap: STACK_SIZE,
            sp: 0,
            fp: 0,
        }
    }

    pub fn push(&mut self, elem: u8) -> Result<(), RuntimeError> {
        if self.sp >= self.cap {
            return Err(RuntimeError::StackOverflow);
        }

        self.stack[self.sp] = elem;
        self.sp += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Result<u8, RuntimeError> {
        if self.sp == 0 {
            Err(RuntimeError::StackUnderflow)
        } else {
            self.sp -= 1;
            Ok(self.stack[self.sp])
        }
    }

    pub fn top(&mut self) -> Option<u8> {
        if self.sp == 0 {
            None
        } else {
            Some(self.stack[self.sp - 1])
        }
    }

    pub fn push_frame(&mut self, locals_count: u8, return_address: u8) -> Result<(), RuntimeError> {
        self.push(return_address)?;
        self.push(self.fp as u8)?;
        self.fp = self.sp;

        for _ in 0..locals_count {
            self.push(0)?;
        }

        Ok(())
    }

    pub fn pop_frame(&mut self) -> Result<u8, RuntimeError> {
        self.sp = self.fp;
        self.fp = self.pop()? as usize;
        self.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push() {
        let mut stack = Stack::new();

        stack.push(10).unwrap();

        assert_eq!(stack.top(), Some(10));
    }

    #[test]
    fn pop() {
        let mut stack = Stack::new();

        stack.push(10).unwrap();
        stack.push(11).unwrap();
        stack.push(12).unwrap();

        assert_eq!(stack.pop(), Ok(12));
        assert_eq!(stack.pop(), Ok(11));
        assert_eq!(stack.pop(), Ok(10));
        assert_eq!(stack.pop(), Err(RuntimeError::StackUnderflow));
    }

    #[test]
    fn push_frame() {
        let mut stack = Stack::new();

        stack.push_frame(3, 12).unwrap();

        assert_eq!(stack.fp, 2);
        assert_eq!(stack.sp, 5);

        assert_eq!(stack.pop(), Ok(0)); // local 1
        assert_eq!(stack.pop(), Ok(0)); // local 2
        assert_eq!(stack.pop(), Ok(0)); // local 3

        assert_eq!(stack.pop(), Ok(0)); // fp

        assert_eq!(stack.pop(), Ok(12)); // return address

        assert_eq!(stack.pop(), Err(RuntimeError::StackUnderflow)); // bottom of stack
    }

    #[test]
    fn pop_frame() {
        let mut stack = Stack::new();

        stack.push_frame(3, 12).unwrap();
        stack.pop_frame().unwrap();

        assert_eq!(stack.sp, 0);
        assert_eq!(stack.fp, 0);
    }
}
