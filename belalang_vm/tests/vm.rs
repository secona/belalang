#![allow(clippy::vec_init_then_push)]
#![allow(clippy::bool_assert_comparison)]

use belalang_vm::mem::stack::StackObject;
use belalang_vm::types::boolean::BelalangBoolean;
use belalang_vm::types::integer::BelalangInteger;
use test_case::test_case;

use belalang_vm::bytecode::{Bytecode, Constant};
use belalang_vm::opcode;
use belalang_vm::vm::VM;

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

        let _ = vm.run(Bytecode {
            instructions,
            constants,
        });

        assert_eq!(vm.stack.size(), 1);

        let Ok(obj) = vm.stack.pop() else { panic!() };
        let StackObject::Object(object) = obj else {
            panic!()
        };
        let Some(int) = object.as_any().downcast_ref::<BelalangInteger>() else {
            panic!()
        };

        assert_eq!(int.value, 12);
    }
}

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

    let _ = vm.run(Bytecode {
        instructions,
        constants,
    });

    assert_eq!(vm.stack.size(), 1);

    let Ok(obj) = vm.stack.pop() else {
        panic!("Failed popping from the stack!");
    };

    let StackObject::Object(object) = obj else {
        panic!("TOS is not an Object!");
    };

    let Some(int) = object.as_any().downcast_ref::<BelalangInteger>() else {
        panic!("Failed downcasting to BelalangInteger!");
    };

    int.value
}

#[test_case(12, 12, opcode::EQUAL => true; "equal")]
#[test_case(12, 12, opcode::NOT_EQUAL => false; "not equal")]
#[test_case(12, 13, opcode::LESS_THAN => true; "less than")]
#[test_case(12, 12, opcode::LESS_THAN_EQUAL => true; "less than equal")]
fn number_comparison_op(a: i64, b: i64, op: u8) -> bool {
    let constants = vec![Constant::Integer(a), Constant::Integer(b)];

    let mut instructions = Vec::new();
    instructions.extend(opcode::constant(0));
    instructions.extend(opcode::constant(1));
    instructions.push(op);

    let mut vm = VM::default();

    let _ = vm.run(Bytecode {
        instructions,
        constants,
    });

    assert_eq!(vm.stack.size(), 1);

    let Ok(obj) = vm.stack.pop() else { panic!() };
    let StackObject::Object(object) = obj else {
        panic!()
    };
    let Some(boolean) = object.as_any().downcast_ref::<BelalangBoolean>() else {
        panic!()
    };

    boolean.value
}

#[test_case(true, true, opcode::EQUAL => true; "equal")]
#[test_case(true, false, opcode::NOT_EQUAL => true; "not equal")]
fn boolean_comparison_op(a: bool, b: bool, op: u8) -> bool {
    let constants = vec![Constant::Boolean(a), Constant::Boolean(b)];

    let mut instructions = Vec::new();
    instructions.extend(opcode::constant(0));
    instructions.extend(opcode::constant(1));
    instructions.push(op);

    let mut vm = VM::default();

    let _ = vm.run(Bytecode {
        instructions,
        constants,
    });

    assert_eq!(vm.stack.size(), 1);

    let Ok(obj) = vm.stack.pop() else { panic!() };
    let StackObject::Object(object) = obj else {
        panic!()
    };
    let Some(boolean) = object.as_any().downcast_ref::<BelalangBoolean>() else {
        panic!()
    };

    boolean.value
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

    let _ = vm.run(Bytecode {
        instructions,
        constants,
    });

    assert_eq!(vm.stack.size(), 1);

    let Ok(obj) = vm.stack.pop() else { panic!() };
    let StackObject::Object(object) = obj else {
        panic!()
    };
    let Some(boolean) = object.as_any().downcast_ref::<BelalangBoolean>() else {
        panic!()
    };

    boolean.value
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

    let _ = vm.run(Bytecode {
        instructions,
        constants,
    });

    assert_eq!(vm.stack.size(), 1);

    let Ok(obj) = vm.stack.pop() else { panic!() };
    let StackObject::Object(object) = obj else {
        panic!()
    };
    let Some(int) = object.as_any().downcast_ref::<BelalangInteger>() else {
        panic!()
    };

    int.value
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

        let _ = vm.run(Bytecode {
            instructions,
            constants,
        });

        assert_eq!(vm.stack.size(), 1);

        let Ok(obj) = vm.stack.pop() else { panic!() };
        let StackObject::Object(object) = obj else {
            panic!()
        };
        let Some(boolean) = object.as_any().downcast_ref::<BelalangBoolean>() else {
            panic!()
        };

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

        let _ = vm.run(Bytecode {
            instructions,
            constants,
        });

        assert_eq!(vm.stack.size(), 2);

        let Ok(obj) = vm.stack.pop() else { panic!() };
        let StackObject::Object(object) = obj else {
            panic!()
        };
        let Some(boolean) = object.as_any().downcast_ref::<BelalangBoolean>() else {
            panic!()
        };

        assert_eq!(boolean.value, false);

        let Ok(obj) = vm.stack.pop() else { panic!() };
        let StackObject::Object(object) = obj else {
            panic!()
        };
        let Some(boolean) = object.as_any().downcast_ref::<BelalangBoolean>() else {
            panic!()
        };

        assert_eq!(boolean.value, true);
    }
}

mod unary_op {
    use super::*;

    #[test]
    fn bang() {
        let constants = Vec::new();

        let mut instructions = Vec::new();
        instructions.push(opcode::TRUE);
        instructions.push(opcode::BANG);

        let mut vm = VM::default();

        let _ = vm.run(Bytecode {
            instructions,
            constants,
        });

        assert_eq!(vm.stack.size(), 1);

        let Ok(obj) = vm.stack.pop() else { panic!() };
        let StackObject::Object(object) = obj else {
            panic!()
        };
        let Some(boolean) = object.as_any().downcast_ref::<BelalangBoolean>() else {
            panic!()
        };

        assert_eq!(boolean.value, false);
    }

    #[test]
    fn minus() {
        let constants = vec![Constant::Integer(12)];

        let mut instructions = Vec::new();
        instructions.extend(opcode::constant(0));
        instructions.push(opcode::MINUS);

        let mut vm = VM::default();

        let _ = vm.run(Bytecode {
            instructions,
            constants,
        });

        assert_eq!(vm.stack.size(), 1);

        let Ok(obj) = vm.stack.pop() else { panic!() };
        let StackObject::Object(object) = obj else {
            panic!()
        };
        let Some(int) = object.as_any().downcast_ref::<BelalangInteger>() else {
            panic!()
        };

        assert_eq!(int.value, -12);
    }
}
