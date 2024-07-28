use std::alloc::{self, Layout};
use std::ptr::{self, NonNull};

pub struct StackAllocator {
    start: NonNull<u8>,
    top: NonNull<u8>,
    len: usize,
    cap: usize,
}

impl StackAllocator {
    pub fn new() -> Self {
        Self {
            start: NonNull::dangling(),
            top: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }

    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<u8>(1).unwrap())
        } else {
            let new_cap = 2 * self.cap;
            let new_layout = Layout::array::<u8>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<u8>(self.cap).unwrap();
            let old_ptr = self.start.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.start = match NonNull::new(new_ptr as *mut u8) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };

        if new_cap == 1 {
            self.top = self.start;
        }

        self.cap = new_cap;
    }

    pub fn push(&mut self, elem: u8) {
        if self.len == self.cap {
            self.grow();
        }

        unsafe {
            ptr::write(self.top.as_ptr(), elem);
        }

        self.len += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use super::StackAllocator;

    #[test]
    fn allocator() {
        let mut allocator = StackAllocator::new();

        allocator.push(10);

        let elem = unsafe { ptr::read(allocator.start.as_ptr()) };
        assert_eq!(elem, 10);
    }
}
