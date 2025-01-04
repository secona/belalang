use belalang_vm::bytecode::Bytecode;
use belalang_vm::object::Object;
use belalang_vm::opcode;
use belalang_vm::vm::VMBuilder;

#[test]
fn simple_push_and_pop() {
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

    let top = vm.stack.top().unwrap();
    assert!(matches!(top, Object::Integer(12)));
}
