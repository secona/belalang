#![allow(clippy::vec_init_then_push)]
#![allow(clippy::bool_assert_comparison)]

use belalang_vm::core::bytecode::{Bytecode, Constant};
use belalang_vm::core::opcode;
use belalang_vm::core::VM;
use belalang_vm::mem::stack::StackObject;
use belalang_vm::objects::boolean::BelalangBoolean;
use belalang_vm::objects::integer::BelalangInteger;

mod stack_op {
    use super::*;

    #[test]
    fn pop() {
        let constants = vec![Constant::Integer(12), Constant::Integer(5)];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));
        instructions.extend(opcode::constant(1));
        instructions.push(opcode::POP);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        })
        .unwrap();

        assert_eq!(vm.stack.size(), 1);

        let Ok(obj) = vm.stack.pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackObject::Object(object) = obj else {
            panic!("TOS is not an Object!");
        };

        let int = unsafe { (object.as_ptr() as *mut BelalangInteger).read() };

        assert_eq!(int.value, 12);
    }
}

mod jump_op {
    use super::*;

    #[test]
    fn jump() {
        let constants = Vec::new();

        let mut instructions = Vec::new();
        instructions.extend(opcode::jump(1));
        instructions.push(opcode::TRUE);
        instructions.push(opcode::FALSE);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        })
        .unwrap();

        assert_eq!(vm.stack.size(), 1);

        let Ok(obj) = vm.stack.pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackObject::Object(object) = obj else {
            panic!("TOS is not an Object!");
        };

        let boolean = unsafe { (object.as_ptr() as *mut BelalangBoolean).read() };
        assert_eq!(boolean.value, false);
    }

    #[test]
    fn jump_if_false_op() {
        let constants = Vec::new();

        let mut instructions = Vec::new();
        instructions.push(opcode::TRUE);
        instructions.extend(opcode::jump_if_false(1));
        instructions.push(opcode::TRUE);
        instructions.push(opcode::FALSE);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        })
        .unwrap();

        assert_eq!(vm.stack.size(), 2);

        let Ok(obj) = vm.stack.pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackObject::Object(object) = obj else {
            panic!("TOS is not an Object!");
        };

        let boolean = unsafe { (object.as_ptr() as *mut BelalangBoolean).read() };

        assert_eq!(boolean.value, false);

        let Ok(obj) = vm.stack.pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackObject::Object(object) = obj else {
            panic!("TOS is not an Object!");
        };

        let boolean = unsafe { (object.as_ptr() as *mut BelalangBoolean).read() };

        assert_eq!(boolean.value, true);
    }
}
