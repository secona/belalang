#![allow(clippy::vec_init_then_push)]
#![allow(clippy::bool_assert_comparison)]

use belalang_vm::core::VM;
use belalang_vm::core::bytecode::{Bytecode, Constant};
use belalang_vm::core::opcode;
use belalang_vm::mem::stack::StackValue;
use belalang_vm::objects::boolean::BelalangBoolean;
use belalang_vm::objects::integer::BelalangInteger;
use belalang_vm::objects::string::BelalangString;

mod number {
    use super::*;

    fn test_arithmetic_op(a: i64, b: i64, op: u8, c: i64) {
        let constants = vec![Constant::Integer(a), Constant::Integer(b)];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));
        instructions.extend(opcode::constant(1));
        instructions.push(op);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        })
        .unwrap();

        assert_eq!(vm.stack_size(), 1, "Stack size is not 1!");

        let Ok(obj) = vm.stack_pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackValue::ObjectPtr(object) = obj else {
            panic!("TOS is not an Object!");
        };

        assert_eq!(
            unsafe { (object.as_ptr() as *mut BelalangInteger).read() }.value,
            c
        );
    }

    #[test]
    fn arithmetic_op_addition() {
        test_arithmetic_op(12, 5, opcode::ADD, 17);
    }

    #[test]
    fn arithmetic_op_subtraction() {
        test_arithmetic_op(12, 5, opcode::SUB, 7);
    }

    #[test]
    fn arithmetic_op_multiplication() {
        test_arithmetic_op(12, 5, opcode::MUL, 60);
    }

    #[test]
    fn arithmetic_op_division() {
        test_arithmetic_op(12, 5, opcode::DIV, 2);
    }

    #[test]
    fn arithmetic_op_modulo() {
        test_arithmetic_op(12, 5, opcode::MOD, 2);
    }

    fn test_comparison_op(a: i64, b: i64, op: u8, c: bool) {
        let constants = vec![Constant::Integer(a), Constant::Integer(b)];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));
        instructions.extend(opcode::constant(1));
        instructions.push(op);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        })
        .unwrap();

        assert_eq!(vm.stack_size(), 1);

        let Ok(obj) = vm.stack_pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackValue::ObjectPtr(object) = obj else {
            panic!("TOS is not an Object!");
        };

        assert_eq!(
            unsafe { (object.as_ptr() as *mut BelalangBoolean).read() }.value,
            c
        );
    }

    #[test]
    fn comparison_op_equal() {
        test_comparison_op(12, 12, opcode::EQUAL, true);
    }

    #[test]
    fn comparison_op_not_equal() {
        test_comparison_op(12, 12, opcode::NOT_EQUAL, false);
    }

    #[test]
    fn comparison_op_less_than() {
        test_comparison_op(12, 13, opcode::LESS_THAN, true);
    }

    #[test]
    fn comparison_op_less_than_equal() {
        test_comparison_op(12, 12, opcode::LESS_THAN_EQUAL, true);
    }

    fn test_bitwise_op(a: i64, b: i64, op: u8, c: i64) {
        let constants = vec![Constant::Integer(a), Constant::Integer(b)];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));
        instructions.extend(opcode::constant(1));
        instructions.push(op);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        })
        .unwrap();

        assert_eq!(vm.stack_size(), 1);

        let Ok(obj) = vm.stack_pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackValue::ObjectPtr(object) = obj else {
            panic!("TOS is not an Object!");
        };

        assert_eq!(
            unsafe { (object.as_ptr() as *mut BelalangInteger).read() }.value,
            c
        )
    }

    #[test]
    fn bitwise_op_bit_and() {
        test_bitwise_op(12, 1, opcode::BIT_AND, 0);
    }

    #[test]
    fn bitwise_op_bit_or() {
        test_bitwise_op(12, 1, opcode::BIT_OR, 13);
    }

    #[test]
    fn bitwise_op_bit_xor() {
        test_bitwise_op(12, 1, opcode::BIT_OR, 13);
    }

    #[test]
    fn bitwise_op_bit_sl() {
        test_bitwise_op(12, 1, opcode::BIT_SL, 24);
    }

    #[test]
    fn bitwise_op_bit_sr() {
        test_bitwise_op(12, 1, opcode::BIT_SR, 6);
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
        })
        .unwrap();

        assert_eq!(vm.stack_size(), 1);

        let Ok(obj) = vm.stack_pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackValue::ObjectPtr(object) = obj else {
            panic!("TOS is not an Object!");
        };

        let int = unsafe { (object.as_ptr() as *mut BelalangInteger).read() };

        assert_eq!(int.value, -12);
    }
}

mod boolean {
    use super::*;

    fn test_comparison_op(a: bool, b: bool, op: u8, c: bool) {
        let constants = vec![Constant::Boolean(a), Constant::Boolean(b)];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));
        instructions.extend(opcode::constant(1));
        instructions.push(op);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        })
        .unwrap();

        assert_eq!(vm.stack_size(), 1);

        let Ok(obj) = vm.stack_pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackValue::ObjectPtr(object) = obj else {
            panic!("TOS is not an Object!");
        };

        assert_eq!(
            unsafe { (object.as_ptr() as *mut BelalangBoolean).read() }.value,
            c
        );
    }

    #[test]
    fn comparison_op_equal() {
        test_comparison_op(true, true, opcode::EQUAL, true);
    }

    #[test]
    fn comparison_op_not_equal() {
        test_comparison_op(true, false, opcode::NOT_EQUAL, true);
    }

    fn test_logical_op(a: bool, b: bool, op: u8, c: bool) {
        let constants = vec![Constant::Boolean(a), Constant::Boolean(b)];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));
        instructions.extend(opcode::constant(1));
        instructions.push(op);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        })
        .unwrap();

        assert_eq!(vm.stack_size(), 1);

        let Ok(obj) = vm.stack_pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackValue::ObjectPtr(object) = obj else {
            panic!("TOS is not an Object!");
        };

        assert_eq!(
            unsafe { (object.as_ptr() as *mut BelalangBoolean).read() }.value,
            c
        );
    }

    #[test]
    fn logical_op_and() {
        test_logical_op(true, false, opcode::AND, false);
    }

    #[test]
    fn logical_op_or() {
        test_logical_op(true, false, opcode::OR, true);
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
        })
        .unwrap();

        assert_eq!(vm.stack_size(), 1);

        let Ok(obj) = vm.stack_pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackValue::ObjectPtr(object) = obj else {
            panic!("TOS is not an Object!");
        };

        let boolean = unsafe { (object.as_ptr() as *mut BelalangBoolean).read() };

        assert_eq!(boolean.value, false);
    }
}

mod string {
    use super::*;

    #[test]
    fn init() {
        let constants = vec![Constant::String("Hello")];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        })
        .unwrap();

        assert_eq!(vm.stack_size(), 1, "Stack size is not 1!");

        let Ok(obj) = vm.stack_pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackValue::ObjectPtr(object) = obj else {
            panic!("TOS is not an Object!");
        };

        let string = unsafe { (object.as_ptr() as *mut BelalangString).read() };
        assert_eq!(format!("{string}"), "Hello");
    }

    fn test_arithmetic_op_mul(
        string: &'static str,
        num: i64,
        expected_string: &str,
        expected_len: usize,
    ) {
        let constants = vec![Constant::String(string), Constant::Integer(num)];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));
        instructions.extend(opcode::constant(1));
        instructions.push(opcode::MUL);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        })
        .unwrap();

        assert_eq!(vm.stack_size(), 1, "Stack size is not 1!");

        let Ok(obj) = vm.stack_pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackValue::ObjectPtr(object) = obj else {
            panic!("TOS is not an Object!");
        };

        let string = unsafe { (object.as_ptr() as *mut BelalangString).read() };

        assert_eq!(format!("{string}"), expected_string);
        assert_eq!(string.len, expected_len);
    }

    #[test]
    fn arithmetic_op_mul_neg_1() {
        test_arithmetic_op_mul("Hello", -1, "", 0);
    }

    #[test]
    fn arithmetic_op_mul_0() {
        test_arithmetic_op_mul("Hello", 0, "", 0);
    }

    #[test]
    fn arithmetic_op_mul_1() {
        test_arithmetic_op_mul("Hello", 1, "Hello", 5);
    }

    #[test]
    fn arithmetic_op_mul_3() {
        test_arithmetic_op_mul("Hello", 3, "HelloHelloHello", 15);
    }

    fn test_arithmetic_op_add(string_1: &'static str, string_2: &'static str, expected: &str) {
        let constants = vec![Constant::String(string_1), Constant::String(string_2)];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));
        instructions.extend(opcode::constant(1));
        instructions.push(opcode::ADD);

        let mut vm = VM::default();

        vm.run(Bytecode {
            instructions,
            constants,
        })
        .unwrap();

        assert_eq!(vm.stack_size(), 1, "Stack size is not 1!");

        let Ok(obj) = vm.stack_pop() else {
            panic!("Failed popping from the stack!");
        };

        let StackValue::ObjectPtr(object) = obj else {
            panic!("TOS is not an Object!");
        };

        let string = unsafe { (object.as_ptr() as *mut BelalangString).read() };

        assert_eq!(format!("{string}"), expected);
    }

    #[test]
    fn arithmetic_op_add_hello_world() {
        test_arithmetic_op_add("Hello", ", World!", "Hello, World!");
    }
}

// mod array {
//     use super::*;
//
//     #[test]
//     fn array() {
//         let constants = vec![
//             Constant::Integer(1),
//             Constant::Integer(2),
//             Constant::Integer(3),
//         ];
//
//         let mut instructions = Vec::new();
//         instructions.extend(opcode::constant(0));
//         instructions.extend(opcode::constant(1));
//         instructions.extend(opcode::constant(2));
//         instructions.extend(opcode::array(3));
//
//         let mut vm = VM::default();
//
//         vm.run(Bytecode {
//             instructions,
//             constants,
//         })
//         .unwrap();
//     }
// }
