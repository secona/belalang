use test_case::test_case;

use belalang_vm::bytecode::Bytecode;
use belalang_vm::object::Object;
use belalang_vm::opcode;
use belalang_vm::vm::VMBuilder;

#[test]
fn push_and_pop() {
    let constants = vec![Object::Integer(12), Object::Integer(5)];

    let mut instructions = Vec::new();
    instructions.extend(opcode::constant(0));
    instructions.extend(opcode::constant(1));
    instructions.push(opcode::POP);

    let mut vm = VMBuilder::default().build();

    let _ = vm.run(Bytecode {
        instructions,
        constants,
    });

    assert_eq!(vm.stack.size(), 1);
    assert!(matches!(vm.stack.top().unwrap(), Object::Integer(12)));
}

#[test_case(12, 5, opcode::ADD => 17; "addition")]
#[test_case(12, 5, opcode::SUB => 7; "subtraction")]
#[test_case(12, 5, opcode::MUL => 60; "multiplication")]
#[test_case(12, 5, opcode::DIV => 2; "division")]
#[test_case(12, 5, opcode::MOD => 2; "modulo")]
fn arithmetic(a: i64, b: i64, op: u8) -> i64 {
    let constants = vec![Object::Integer(a), Object::Integer(b)];

    let mut instructions = Vec::new();
    instructions.extend(opcode::constant(0));
    instructions.extend(opcode::constant(1));
    instructions.push(op);

    let mut vm = VMBuilder::default().build();

    let _ = vm.run(Bytecode {
        instructions,
        constants,
    });

    assert_eq!(vm.stack.size(), 1);

    if let Object::Integer(result) = vm.stack.top().unwrap() {
        return *result;
    }

    panic!("Not an Integer!");
}

#[test_case(12, 12, opcode::EQUAL => true; "equal")]
#[test_case(12, 12, opcode::NOT_EQUAL => false; "not equal")]
#[test_case(12, 13, opcode::LESS_THAN => true; "less than")]
#[test_case(12, 12, opcode::LESS_THAN_EQUAL => true; "less than equal")]
fn number_equality(a: i64, b: i64, op: u8) -> bool {
    let constants = vec![Object::Integer(a), Object::Integer(b)];

    let mut instructions = Vec::new();
    instructions.extend(opcode::constant(0));
    instructions.extend(opcode::constant(1));
    instructions.push(op);

    let mut vm = VMBuilder::default().build();

    let _ = vm.run(Bytecode {
        instructions,
        constants,
    });

    assert_eq!(vm.stack.size(), 1);

    if let Object::Boolean(result) = vm.stack.top().unwrap() {
        return *result;
    }

    panic!("Not an Boolean!");
}

#[test_case(true, false, opcode::AND => false; "and")]
#[test_case(true, false, opcode::OR => true; "or")]
fn logical(a: bool, b: bool, op: u8) -> bool {
    let constants = vec![Object::Boolean(a), Object::Boolean(b)];

    let mut instructions = Vec::new();
    instructions.extend(opcode::constant(0));
    instructions.extend(opcode::constant(1));
    instructions.push(op);

    let mut vm = VMBuilder::default().build();

    let _ = vm.run(Bytecode {
        instructions,
        constants,
    });

    assert_eq!(vm.stack.size(), 1);

    if let Object::Boolean(result) = vm.stack.top().unwrap() {
        return *result;
    }

    panic!("Not an Boolean!");
}
