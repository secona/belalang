#![allow(clippy::vec_init_then_push)]
#![allow(clippy::bool_assert_comparison)]

use belalang_vm::mem::stack::StackObject;
use belalang_vm::types::boolean::BelalangBoolean;
use belalang_vm::types::integer::BelalangInteger;
use belalang_vm::types::string::BelalangString;
use belalang_vm::bytecode::{Bytecode, Constant};
use belalang_vm::opcode;
use belalang_vm::vm::VM;

mod number {
    use super::*;
    use test_case::test_case;

    #[test_case(12, 5, opcode::ADD => 17; "addition")]
    #[test_case(12, 5, opcode::SUB => 7; "subtraction")]
    #[test_case(12, 5, opcode::MUL => 60; "multiplication")]
    #[test_case(12, 5, opcode::DIV => 2; "division")]
    #[test_case(12, 5, opcode::MOD => 2; "modulo")]
    fn arithmetic_op(a: i64, b: i64, op: u8) -> i64 {
        let constants = vec![Constant::Integer(a), Constant::Integer(b)];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));
        instructions.extend(opcode::constant(1));
        instructions.push(op);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        }).unwrap();

        assert_eq!(vm.stack.size(), 1, "Stack size is not 1!");

        let Ok(obj) = vm.stack.pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackObject::Object(object) = obj else {
            panic!("TOS is not an Object!");
        };

        unsafe { (object.as_ptr() as *mut BelalangInteger).read() }.value
    }

    #[test_case(12, 12, opcode::EQUAL => true; "equal")]
    #[test_case(12, 12, opcode::NOT_EQUAL => false; "not equal")]
    #[test_case(12, 13, opcode::LESS_THAN => true; "less than")]
    #[test_case(12, 12, opcode::LESS_THAN_EQUAL => true; "less than equal")]
    fn comparison_op(a: i64, b: i64, op: u8) -> bool {
        let constants = vec![Constant::Integer(a), Constant::Integer(b)];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));
        instructions.extend(opcode::constant(1));
        instructions.push(op);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        }).unwrap();

        assert_eq!(vm.stack.size(), 1);

        let Ok(obj) = vm.stack.pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackObject::Object(object) = obj else {
            panic!("TOS is not an Object!");
        };

        unsafe { (object.as_ptr() as *mut BelalangBoolean).read() }.value
    }

    #[test_case(12, 1, opcode::BIT_AND => 0; "bit and")]
    #[test_case(12, 1, opcode::BIT_OR => 13; "bit or")]
    #[test_case(12, 1, opcode::BIT_XOR => 13; "bit xor")]
    #[test_case(12, 1, opcode::BIT_SL => 24; "bit sl")]
    #[test_case(12, 1, opcode::BIT_SR => 6; "bit sr")]
    fn bitwise_op(a: i64, b: i64, op: u8) -> i64 {
        let constants = vec![Constant::Integer(a), Constant::Integer(b)];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));
        instructions.extend(opcode::constant(1));
        instructions.push(op);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        }).unwrap();

        assert_eq!(vm.stack.size(), 1);

        let Ok(obj) = vm.stack.pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackObject::Object(object) = obj else {
            panic!("TOS is not an Object!");
        };

        unsafe { (object.as_ptr() as *mut BelalangInteger).read() }.value
    }

    #[test]
    fn minus() {
        let constants = vec![Constant::Integer(12)];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));
        instructions.push(opcode::MINUS);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        }).unwrap();

        assert_eq!(vm.stack.size(), 1);

        let Ok(obj) = vm.stack.pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackObject::Object(object) = obj else {
            panic!("TOS is not an Object!");
        };

        let int = unsafe { (object.as_ptr() as *mut BelalangInteger).read() };

        assert_eq!(int.value, -12);
    }
}

mod boolean {
    use super::*;
    use test_case::test_case;

    #[test_case(true, true, opcode::EQUAL => true; "equal")]
    #[test_case(true, false, opcode::NOT_EQUAL => true; "not equal")]
    fn comparison_op(a: bool, b: bool, op: u8) -> bool {
        let constants = vec![Constant::Boolean(a), Constant::Boolean(b)];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));
        instructions.extend(opcode::constant(1));
        instructions.push(op);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        }).unwrap();

        assert_eq!(vm.stack.size(), 1);

        let Ok(obj) = vm.stack.pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackObject::Object(object) = obj else {
            panic!("TOS is not an Object!");
        };

        unsafe { (object.as_ptr() as *mut BelalangBoolean).read() }.value
    }

    #[test_case(true, false, opcode::AND => false; "and")]
    #[test_case(true, false, opcode::OR => true; "or")]
    fn logical_op(a: bool, b: bool, op: u8) -> bool {
        let constants = vec![Constant::Boolean(a), Constant::Boolean(b)];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));
        instructions.extend(opcode::constant(1));
        instructions.push(op);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        }).unwrap();

        assert_eq!(vm.stack.size(), 1);

        let Ok(obj) = vm.stack.pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackObject::Object(object) = obj else {
            panic!("TOS is not an Object!");
        };

        unsafe { (object.as_ptr() as *mut BelalangBoolean).read() }.value
    }


    #[test]
    fn bang() {
        let constants = Vec::new();

        let mut instructions = Vec::new();
        instructions.push(opcode::TRUE);
        instructions.push(opcode::BANG);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        }).unwrap();

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
}

mod string {
    use super::*;
    use test_case::test_case;

    #[test]
    fn init() {
        let constants = vec![Constant::String("Hello")];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        }).unwrap();

        assert_eq!(vm.stack.size(), 1, "Stack size is not 1!");

        let Ok(obj) = vm.stack.pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackObject::Object(object) = obj else {
            panic!("TOS is not an Object!");
        };

        let string = unsafe { (object.as_ptr() as *mut BelalangString).read() };
        assert_eq!(format!("{string}"), "Hello");
    }

    #[test_case("Hello", -1 => (String::from(""), 0); "mul neg 1")]
    #[test_case("Hello", 0 => (String::from(""), 0); "mul 0")]
    #[test_case("Hello", 1 => (String::from("Hello"), 5); "mul 1")]
    #[test_case("Hello", 3 => (String::from("HelloHelloHello"), 15); "mul 3")]
    fn arithmetic_op_mul(string: &'static str, num: i64) -> (String, usize) {
        let constants = vec![Constant::String(string), Constant::Integer(num)];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));
        instructions.extend(opcode::constant(1));
        instructions.push(opcode::MUL);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        }).unwrap();

        assert_eq!(vm.stack.size(), 1, "Stack size is not 1!");

        let Ok(obj) = vm.stack.pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackObject::Object(object) = obj else {
            panic!("TOS is not an Object!");
        };

        let string = unsafe { (object.as_ptr() as *mut BelalangString).read() };

        (format!("{string}"), string.len)
    }

    #[test_case("Hello", ", World!" => String::from("Hello, World!"); "hello world")]
    fn arithmetic_op_add(string_1: &'static str, string_2: &'static str) -> String {
        let constants = vec![Constant::String(string_1), Constant::String(string_2)];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));
        instructions.extend(opcode::constant(1));
        instructions.push(opcode::ADD);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        }).unwrap();

        assert_eq!(vm.stack.size(), 1, "Stack size is not 1!");

        let Ok(obj) = vm.stack.pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackObject::Object(object) = obj else {
            panic!("TOS is not an Object!");
        };

        let string = unsafe { (object.as_ptr() as *mut BelalangString).read() };

        format!("{string}")
    }
}

mod array {
    use super::*;

    #[test]
    fn array() {
        let constants = vec![
            Constant::Integer(1),
            Constant::Integer(2),
            Constant::Integer(3),
        ];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));
        instructions.extend(opcode::constant(1));
        instructions.extend(opcode::constant(2));
        instructions.extend(opcode::array(3));

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        }).unwrap();
    }
}
