const STACK_SIZE: usize = 4096;

pub struct StackAllocator {
    stack: [u8; STACK_SIZE],
    cap: usize,
    sp: usize,
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
        }
    }

    pub fn push(&mut self, elem: u8) {
        if self.sp >= self.cap {
            panic!("stack overflow");
        }

        self.stack[self.sp] = elem;
        self.sp += 1;
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.sp == 0 {
            None
        } else {
            self.sp -= 1;
            Some(self.stack[self.sp])
        }
    }

    pub fn top(&mut self) -> Option<u8> {
        if self.sp == 0 {
            None
        } else {
            Some(self.stack[self.sp - 1])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::StackAllocator;

    #[test]
    fn allocator_push() {
        let mut allocator = StackAllocator::new();

        allocator.push(10);

        assert_eq!(allocator.top(), Some(10));
    }

    #[test]
    fn allocator_pop() {
        let mut allocator = StackAllocator::new();

        allocator.push(10);
        allocator.push(11);
        allocator.push(12);

        assert_eq!(allocator.pop(), Some(12));
        assert_eq!(allocator.pop(), Some(11));
        assert_eq!(allocator.pop(), Some(10));
        assert_eq!(allocator.pop(), None);
    }
}
