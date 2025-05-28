use crate::core::BelalangPtr;
use crate::errors::RuntimeError;

/// Default stack size of Belalang VM
///
/// This is currently a good enough stack size for Belalang VM. I don't really know how stack sizes
/// are supposed to be implemented for efficiency. I still have stuff to read and things to
/// explore.
const STACK_SIZE: usize = 4096;

/// Values that live on the stack
#[derive(Default, Debug)]
pub enum StackValue {
    /// Pointer to an object, like [`BelalangInteger`][crate::objects::integer::BelalangInteger]
    ObjectPtr(BelalangPtr),

    /// Pointer to an address in the bytecode
    AddressPtr(u8),

    /// Null value in the stack
    ///
    /// This value is mostly used to indicate uninitialized variables and actual null values.
    ///
    /// # Problems
    /// - I am not sure to go forward with the "null" name or not.
    /// - I am not sure if uninitialized variables should have this as their value.
    #[default]
    Null,
}

/// Belalang VM's stack implementation
///
/// This stack is both the call stack and the frame stack.
pub struct Stack {
    stack: [StackValue; STACK_SIZE],
    cap: usize,
    sp: usize,
    fp: usize,
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        while self.pop().is_ok() {}
    }
}

impl Stack {
    /// Creates a new stack
    ///
    /// Pretty self explainatory.
    pub fn new() -> Self {
        Self {
            stack: [const { StackValue::Null }; STACK_SIZE],
            cap: STACK_SIZE,
            sp: 0,
            fp: 0,
        }
    }

    /// Returns the stack size
    pub fn size(&self) -> usize {
        self.sp
    }

    /// Pushes a new [`StackValue`] to the stack
    pub fn push(&mut self, elem: StackValue) -> Result<(), RuntimeError> {
        if self.sp >= self.cap {
            return Err(RuntimeError::StackOverflow);
        }

        self.stack[self.sp] = elem;
        self.sp += 1;

        Ok(())
    }

    /// Pops a [`StackValue`] from the stack
    ///
    /// This function uses [`std::mem::take`] to get the top-most value of the stack, leaving a
    /// [`StackValue::Null`] behind.
    pub fn pop(&mut self) -> Result<StackValue, RuntimeError> {
        if self.sp == 0 {
            Err(RuntimeError::StackUnderflow)
        } else {
            self.sp -= 1;
            Ok(std::mem::take(&mut self.stack[self.sp]))
        }
    }

    /// Gets the top-most [`StackValue`] value from the stack
    ///
    /// Returns the reference to the top-most value, and does not remove it.
    pub fn top(&mut self) -> Option<&StackValue> {
        if self.sp == 0 {
            None
        } else {
            Some(&self.stack[self.sp - 1])
        }
    }

    /// Pushes a new stack frame to the stack
    ///
    /// Typically used when going into a function scope.
    pub fn push_frame(&mut self, locals_count: u8, return_address: u8) -> Result<(), RuntimeError> {
        self.push(StackValue::AddressPtr(return_address))?;
        self.push(StackValue::AddressPtr(self.fp as u8))?;
        self.fp = self.sp;

        for _ in 0..locals_count {
            self.push(StackValue::Null)?;
        }

        Ok(())
    }

    /// Pops a stack frame from the stack
    ///
    /// Typically used when going out of a function scope
    pub fn pop_frame(&mut self) -> Result<StackValue, RuntimeError> {
        self.sp = self.fp;

        if let StackValue::AddressPtr(v) = self.pop()? {
            self.fp = v.into();
        }

        self.pop()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_allocation)]

    use crate::mem::heap::Heap;
    use crate::objects::integer::BelalangInteger;

    use super::*;

    macro_rules! assert_belalang_integer {
        ($top:expr, $value:expr) => {
            let StackValue::ObjectPtr(obj) = $top else {
                panic!("Not a StackValue::Object");
            };

            let int = unsafe { (obj.as_ptr() as *const BelalangInteger).read() };
            assert_eq!(int.value, $value);
        };
    }

    #[test]
    fn push() {
        let mut stack = Stack::new();
        let mut heap = Heap::default();

        let ptr = heap.alloc(BelalangInteger::new(10)).unwrap();
        stack.push(StackValue::ObjectPtr(ptr)).unwrap();

        assert_belalang_integer!(&stack.top().unwrap(), 10);

        drop(stack);
        drop(heap);
    }

    #[test]
    fn pop() {
        let mut stack = Stack::new();
        let mut heap = Heap::default();

        let ptr = heap.alloc(BelalangInteger::new(10)).unwrap();
        stack.push(StackValue::ObjectPtr(ptr)).unwrap();

        let ptr = heap.alloc(BelalangInteger::new(11)).unwrap();
        stack.push(StackValue::ObjectPtr(ptr)).unwrap();

        let ptr = heap.alloc(BelalangInteger::new(12)).unwrap();
        stack.push(StackValue::ObjectPtr(ptr)).unwrap();

        assert_belalang_integer!(&stack.pop().unwrap(), 12);
        assert_belalang_integer!(&stack.pop().unwrap(), 11);
        assert_belalang_integer!(&stack.pop().unwrap(), 10);

        assert!(matches!(stack.pop(), Err(RuntimeError::StackUnderflow)));
    }

    #[test]
    fn push_frame() {
        let mut stack = Stack::new();

        stack.push_frame(3, 12).unwrap();

        assert_eq!(stack.fp, 2);
        assert_eq!(stack.sp, 5);

        assert!(matches!(stack.pop().unwrap(), StackValue::Null)); // local 1
        assert!(matches!(stack.pop().unwrap(), StackValue::Null)); // local 2
        assert!(matches!(stack.pop().unwrap(), StackValue::Null)); // local 3
        assert!(matches!(stack.pop().unwrap(), StackValue::AddressPtr(0))); // fp
        assert!(matches!(stack.pop().unwrap(), StackValue::AddressPtr(12))); // return address
        assert!(matches!(stack.pop(), Err(RuntimeError::StackUnderflow))); // bottom of stack
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
