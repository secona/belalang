use belalang_vm::opcode;

fn read_u16(bytes: &Vec<u8>, i: &mut usize) -> (u16, usize) {
    let hi = bytes[*i + 1];
    let lo = bytes[*i + 2];
    *i += 2;

    ((hi as u16) << 8 | lo as u16, *i - 2)
}

fn read_u8(bytes: &Vec<u8>, i: &mut usize) -> (u16, usize) {
    let value = bytes[*i + 1];
    *i += 1;

    (value as u16, *i - 1)
}

pub fn disassemble(bytes: Vec<u8>) -> String {
    let mut result = String::new();
    let mut i = 0;

    while i < bytes.len() {
        match bytes[i] {
            opcode::CONSTANT => {
                let (operand, start_i) = read_u16(&bytes, &mut i);
                result.push_str(&format!("{:#06x}: CONSTANT {:#03}\n", start_i, operand));
            }

            opcode::POP => {
                result.push_str(&format!("{:#06x}: POP\n", i));
            }

            opcode::ADD => {
                result.push_str(&format!("{:#06x}: ADD\n", i));
            }

            opcode::SUB => {
                result.push_str(&format!("{:#06x}: SUB\n", i));
            }

            opcode::MUL => {
                result.push_str(&format!("{:#06x}: MUL\n", i));
            }

            opcode::DIV => {
                result.push_str(&format!("{:#06x}: DIV\n", i));
            }

            opcode::MOD => {
                result.push_str(&format!("{:#06x}: MOD\n", i));
            }

            opcode::TRUE => {
                result.push_str(&format!("{:#06x}: TRUE\n", i));
            }

            opcode::FALSE => {
                result.push_str(&format!("{:#06x}: FALSE\n", i));
            }

            opcode::NULL => {
                result.push_str(&format!("{:#06x}: NULL\n", i));
            }

            opcode::EQUAL => {
                result.push_str(&format!("{:#06x}: EQUAL\n", i));
            }

            opcode::NOT_EQUAL => {
                result.push_str(&format!("{:#06x}: NOT_EQUAL\n", i));
            }

            opcode::LESS_THAN => {
                result.push_str(&format!("{:#06x}: LESS_THAN\n", i));
            }

            opcode::LESS_THAN_EQUAL => {
                result.push_str(&format!("{:#06x}: LESS_THAN_EQUAL\n", i));
            }

            opcode::BANG => {
                result.push_str(&format!("{:#06x}: BANG\n", i));
            }

            opcode::MINUS => {
                result.push_str(&format!("{:#06x}: MINUS\n", i));
            }

            opcode::JUMP => {
                let (operand, start_i) = read_u16(&bytes, &mut i);
                result.push_str(&format!("{:#06x}: JUMP {:#03}\n", start_i, operand));
            }
            opcode::JUMP_IF_FALSE => {
                let (operand, start_i) = read_u16(&bytes, &mut i);
                result.push_str(&format!(
                    "{:#06x}: JUMP_IF_FALSE {:#03}\n",
                    start_i, operand
                ));
            }

            opcode::SET_GLOBAL => {
                let (operand, start_i) = read_u16(&bytes, &mut i);
                result.push_str(&format!("{:#06x}: SET_GLOBAL {:#03}\n", start_i, operand));
            }

            opcode::GET_GLOBAL => {
                let (operand, start_i) = read_u16(&bytes, &mut i);
                result.push_str(&format!("{:#06x}: GET_GLOBAL {:#03}\n", start_i, operand));
            }

            opcode::SET_LOCAL => {
                let (operand, start_i) = read_u8(&bytes, &mut i);
                result.push_str(&format!("{:#06x}: SET_LOCAL {:#03}\n", start_i, operand));
            }

            opcode::GET_LOCAL => {
                let (operand, start_i) = read_u8(&bytes, &mut i);
                result.push_str(&format!("{:#06x}: GET_LOCAL {:#03}\n", start_i, operand));
            }

            opcode::GET_BUILTIN => {
                let (operand, start_i) = read_u8(&bytes, &mut i);
                result.push_str(&format!("{:#06x}: GET_BUILTIN {:#03}\n", start_i, operand));
            }

            opcode::CALL => {
                result.push_str(&format!("{:#06x}: CALL\n", i));
            }

            opcode::RETURN => {
                result.push_str(&format!("{:#06x}: RETURN\n", i));
            }

            opcode::RETURN_VALUE => {
                result.push_str(&format!("{:#06x}: RETURN\n", i));
            }

            _ => {}
        }

        i += 1;
    }

    result
}
