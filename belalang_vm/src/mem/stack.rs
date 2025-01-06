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

    pub fn push_frame(&mut self, locals_count: u8, return_address: u8) {
        self.push(return_address);
        self.push(self.fp as u8);
        self.fp = self.sp;

        for _ in 0..locals_count {
            self.push(0);
        }
    }

    pub fn pop_frame(&mut self) -> Option<u8> {
        self.sp = self.fp;
        self.fp = self.pop()? as usize;
        self.pop()
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

    #[test]
    fn allocator_push_frame() {
        let mut allocator = StackAllocator::new();

        allocator.push_frame(3, 12);

        assert_eq!(allocator.fp, 2);
        assert_eq!(allocator.sp, 5);

        assert_eq!(allocator.pop(), Some(0)); // local 1
        assert_eq!(allocator.pop(), Some(0)); // local 2
        assert_eq!(allocator.pop(), Some(0)); // local 3

        assert_eq!(allocator.pop(), Some(0)); // fp

        assert_eq!(allocator.pop(), Some(12)); // return address
    }

    #[test]
    fn allocator_pop_frame() {
        let mut allocator = StackAllocator::new();

        allocator.push_frame(3, 12);
        allocator.pop_frame();

        assert_eq!(allocator.sp, 0);
        assert_eq!(allocator.fp, 0);
    }
}
