use crate::types::BelalangType;
use crate::errors::RuntimeError;

const STACK_SIZE: usize = 4096;

/// This is subject to change. I resort to this
/// because I haven't implemented the heap yet.
#[derive(Default, Debug)]
pub enum StackObject {
    Object(Box<dyn BelalangType>),
    Ptr(u8),
    #[default]
    Null,
}

const DEFAULT_STACK_VALUE: StackObject = StackObject::Null;

pub struct Stack {
    stack: [StackObject; STACK_SIZE],
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
            stack: [DEFAULT_STACK_VALUE; STACK_SIZE],
            cap: STACK_SIZE,
            sp: 0,
            fp: 0,
        }
    }

    pub fn size(&self) -> usize {
        self.sp
    }

    pub fn push(&mut self, elem: StackObject) -> Result<(), RuntimeError> {
        if self.sp >= self.cap {
            return Err(RuntimeError::StackOverflow);
        }

        self.stack[self.sp] = elem;
        self.sp += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Result<StackObject, RuntimeError> {
        if self.sp == 0 {
            Err(RuntimeError::StackUnderflow)
        } else {
            self.sp -= 1;
            Ok(std::mem::take(&mut self.stack[self.sp]))
        }
    }

    pub fn top(&mut self) -> Option<&StackObject> {
        if self.sp == 0 {
            None
        } else {
            Some(&self.stack[self.sp - 1])
        }
    }

    pub fn push_frame(&mut self, locals_count: u8, return_address: u8) -> Result<(), RuntimeError> {
        self.push(StackObject::Ptr(return_address))?;
        self.push(StackObject::Ptr(self.fp as u8))?;
        self.fp = self.sp;

        for _ in 0..locals_count {
            self.push(StackObject::Null)?;
        }

        Ok(())
    }

    pub fn pop_frame(&mut self) -> Result<StackObject, RuntimeError> {
        self.sp = self.fp;

        if let StackObject::Ptr(v) = self.pop()? {
            self.fp = v.into();
        }

        self.pop()
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused_allocation)]

    use crate::types::integer::BelalangInteger;

    use super::*;

    macro_rules! assert_belalang_integer {
        ($top:expr, $value:expr) => {
            assert!(matches!(
                $top,
                StackObject::Object(obj) if *obj == Box::new(BelalangInteger($value))
            ));
        };
    }

    #[test]
    fn push() {
        let mut stack = Stack::new();

        stack
            .push(StackObject::Object(Box::new(BelalangInteger(10))))
            .unwrap();

        assert_belalang_integer!(stack.top().unwrap(), 10);
    }

    #[test]
    fn pop() {
        let mut stack = Stack::new();

        stack
            .push(StackObject::Object(Box::new(BelalangInteger(10))))
            .unwrap();
        stack
            .push(StackObject::Object(Box::new(BelalangInteger(11))))
            .unwrap();
        stack
            .push(StackObject::Object(Box::new(BelalangInteger(12))))
            .unwrap();

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

        assert!(matches!(stack.pop().unwrap(), StackObject::Null)); // local 1
        assert!(matches!(stack.pop().unwrap(), StackObject::Null)); // local 2
        assert!(matches!(stack.pop().unwrap(), StackObject::Null)); // local 3
        assert!(matches!(stack.pop().unwrap(), StackObject::Ptr(0))); // fp
        assert!(matches!(stack.pop().unwrap(), StackObject::Ptr(12))); // return address
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
