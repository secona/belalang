use crate::error::RuntimeError;

const STACK_SIZE: usize = 4096;

pub struct StackAllocator {
    stack: [u8; STACK_SIZE],
    cap: usize,
    sp: usize,
    fp: usize,
}

impl Default for StackAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl StackAllocator {
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
    fn allocator_push() {
        let mut allocator = StackAllocator::new();

        allocator.push(10).unwrap();

        assert_eq!(allocator.top(), Some(10));
    }

    #[test]
    fn allocator_pop() {
        let mut allocator = StackAllocator::new();

        allocator.push(10).unwrap();
        allocator.push(11).unwrap();
        allocator.push(12).unwrap();

        assert_eq!(allocator.pop(), Ok(12));
        assert_eq!(allocator.pop(), Ok(11));
        assert_eq!(allocator.pop(), Ok(10));
        assert_eq!(allocator.pop(), Err(RuntimeError::StackUnderflow));
    }

    #[test]
    fn allocator_push_frame() {
        let mut allocator = StackAllocator::new();

        allocator.push_frame(3, 12).unwrap();

        assert_eq!(allocator.fp, 2);
        assert_eq!(allocator.sp, 5);

        assert_eq!(allocator.pop(), Ok(0)); // local 1
        assert_eq!(allocator.pop(), Ok(0)); // local 2
        assert_eq!(allocator.pop(), Ok(0)); // local 3

        assert_eq!(allocator.pop(), Ok(0)); // fp

        assert_eq!(allocator.pop(), Ok(12)); // return address

        assert_eq!(allocator.pop(), Err(RuntimeError::StackUnderflow)); // bottom of stack
    }

    #[test]
    fn allocator_pop_frame() {
        let mut allocator = StackAllocator::new();

        allocator.push_frame(3, 12).unwrap();
        allocator.pop_frame().unwrap();

        assert_eq!(allocator.sp, 0);
        assert_eq!(allocator.fp, 0);
    }
}
