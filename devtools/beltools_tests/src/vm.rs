use belvm::VM;
use belvm::mem::stack::StackValue;
use belvm::objects::boolean::BelalangBoolean;
use belvm::objects::integer::BelalangInteger;
use belvm_bytecode::{Bytecode, Constant};

#[derive(Default)]
pub struct VMBuilder {
    instructions: Vec<u8>,
    constants: Vec<Constant>,
}

impl VMBuilder {
    pub fn with_instructions(mut self, instructions: Vec<u8>) -> Self {
        self.instructions = instructions;
        self
    }

    pub fn with_constants(mut self, constants: Vec<Constant>) -> Self {
        self.constants = constants;
        self
    }

    pub fn run_ok(self) -> VMRunner {
        let mut vm = VM::default();
        let result = vm.run(Bytecode {
            instructions: self.instructions,
            constants: self.constants,
        });

        result.expect("VM failed to run");
        VMRunner { vm }
    }
}

pub struct VMRunner {
    vm: VM,
}

impl VMRunner {
    pub fn expect_stack_size(self, expected: usize) -> Self {
        assert_eq!(self.vm.stack_size(), expected);
        self
    }

    pub fn expect_stack_top_is_int(mut self, expected: i64) -> Self {
        let obj = self.vm.stack_pop().expect("Failed popping from the stack!");
        let StackValue::ObjectPtr(object) = obj else {
            panic!("TOS is not an ObjectPtr!");
        };
        let int = unsafe { (object.as_ptr() as *mut BelalangInteger).read() };
        assert_eq!(int.value, expected, "Integer value mismatch on stack top!");
        self
    }

    pub fn expect_stack_top_is_bool(mut self, expected: bool) -> Self {
        let obj = self.vm.stack_pop().expect("Failed popping from the stack!");
        let StackValue::ObjectPtr(object) = obj else {
            panic!("TOS is not an ObjectPtr!");
        };
        let boolean = unsafe { (object.as_ptr() as *mut BelalangBoolean).read() };
        assert_eq!(boolean.value, expected, "Boolean value mismatch on stack top!");
        self
    }

    pub fn into_vm(self) -> VM {
        self.vm
    }
}
