use beltools_tests::IntoInstructionBytes;
use beltools_tests::instructions;
use belvm_bytecode::Constant;
use belvm_bytecode::opcode;

mod stack_op {
    use super::*;

    #[test]
    fn pop() {
        let constants = vec![Constant::Integer(12), Constant::Integer(5)];

        let instructions = instructions![opcode::constant(0), opcode::constant(1), opcode::POP,];

        beltools_tests::VMBuilder::default()
            .with_instructions(instructions)
            .with_constants(constants)
            .run_ok()
            .expect_stack_size(1)
            .expect_stack_top_is_int(12);
    }
}

mod jump_op {
    use super::*;

    #[test]
    fn jump() {
        let constants = Vec::new();

        let instructions = instructions![opcode::jump(1), opcode::TRUE, opcode::FALSE,];

        beltools_tests::VMBuilder::default()
            .with_instructions(instructions)
            .with_constants(constants)
            .run_ok()
            .expect_stack_size(1)
            .expect_stack_top_is_bool(false);
    }

    #[test]
    fn jump_if_false_op() {
        let constants = Vec::new();

        let instructions = instructions![opcode::TRUE, opcode::jump_if_false(1), opcode::TRUE, opcode::FALSE,];

        beltools_tests::VMBuilder::default()
            .with_instructions(instructions)
            .with_constants(constants)
            .run_ok()
            .expect_stack_size(2)
            .expect_stack_top_is_bool(false)
            .expect_stack_top_is_bool(true);
    }
}
