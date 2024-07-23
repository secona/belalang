pub const CONSTANT: u8 = 0x00;

pub const POP: u8 = 0x01;

pub const ADD: u8 = 0x02;
pub const SUB: u8 = 0x03;
pub const MUL: u8 = 0x04;
pub const DIV: u8 = 0x05;
pub const MOD: u8 = 0x06;

pub const TRUE: u8 = 0x07;
pub const FALSE: u8 = 0x08;
pub const NULL: u8 = 0x09;

pub const EQUAL: u8 = 0x0A;
pub const NOT_EQUAL: u8 = 0x0B;
pub const LESS_THAN: u8 = 0x0C;
pub const LESS_THAN_EQUAL: u8 = 0x0D;
pub const AND: u8 = 0x0E;
pub const OR: u8 = 0x0F;

pub const BIT_AND: u8 = 0x10;
pub const BIT_OR: u8 = 0x11;
pub const BIT_XOR: u8 = 0x12;
pub const BIT_SL: u8 = 0x13;
pub const BIT_SR: u8 = 0x14;

pub const BANG: u8 = 0x15;
pub const MINUS: u8 = 0x16;

pub const JUMP: u8 = 0x17;
pub const JUMP_IF_FALSE: u8 = 0x18;

pub const SET_GLOBAL: u8 = 0x19;
pub const GET_GLOBAL: u8 = 0x1A;

pub const SET_LOCAL: u8 = 0x1B;
pub const GET_LOCAL: u8 = 0x1C;

pub const GET_BUILTIN: u8 = 0x1D;

pub const CALL: u8 = 0x1E;
pub const RETURN: u8 = 0x1F;
pub const RETURN_VALUE: u8 = 0x20;

pub const ARRAY: u8 = 0x21;
pub const INDEX: u8 = 0x22;

pub fn constant(v: u16) -> [u8; 3] {
    [CONSTANT, (v >> 8) as u8, (v & 0xFF) as u8]
}

pub fn jump(v: u16) -> [u8; 3] {
    [JUMP, (v >> 8) as u8, (v & 0xFF) as u8]
}

pub fn jump_if_false(v: u16) -> [u8; 3] {
    [JUMP_IF_FALSE, (v >> 8) as u8, (v & 0xFF) as u8]
}

pub fn set_global(v: u16) -> [u8; 3] {
    [SET_GLOBAL, (v >> 8) as u8, (v & 0xFF) as u8]
}

pub fn get_global(v: u16) -> [u8; 3] {
    [GET_GLOBAL, (v >> 8) as u8, (v & 0xFF) as u8]
}

pub fn set_local(v: u8) -> [u8; 2] {
    [SET_LOCAL, v]
}

pub fn get_local(v: u8) -> [u8; 2] {
    [GET_LOCAL, v]
}

pub fn get_builtin(v: u8) -> [u8; 2] {
    [GET_BUILTIN, v]
}

pub fn array(v: u16) -> [u8; 3] {
    [ARRAY, (v >> 8) as u8, (v & 0xFF) as u8]
}

#[cfg(test)]
mod tests {
    use crate::opcode;

    #[test]
    fn constant() {
        let bytes = opcode::constant(65534);

        assert_eq!(bytes.len(), 3);
        assert_eq!(bytes[0], 0);
        assert_eq!(bytes[1], 255);
        assert_eq!(bytes[2], 254);
    }
}
