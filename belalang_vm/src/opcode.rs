pub const NOOP: u8 = 0x00;

// Stack Operations
pub const POP: u8 = 0x01;

// Arithmetic Operations
pub const ADD: u8 = 0x10;
pub const SUB: u8 = 0x11;
pub const MUL: u8 = 0x12;
pub const DIV: u8 = 0x13;
pub const MOD: u8 = 0x14;

// Constants
pub const CONSTANT: u8 = 0x20;
pub const TRUE: u8 = 0x21;
pub const FALSE: u8 = 0x22;
pub const NULL: u8 = 0x23;

// Comparison Operations
pub const EQUAL: u8 = 0x30;
pub const NOT_EQUAL: u8 = 0x31;
pub const LESS_THAN: u8 = 0x32;
pub const LESS_THAN_EQUAL: u8 = 0x33;

// Logical Operations
pub const AND: u8 = 0x40;
pub const OR: u8 = 0x41;

// Bitwise Operations
pub const BIT_AND: u8 = 0x50;
pub const BIT_OR: u8 = 0x51;
pub const BIT_XOR: u8 = 0x52;
pub const BIT_SL: u8 = 0x53;
pub const BIT_SR: u8 = 0x54;

// Unary Operations
pub const BANG: u8 = 0x60;
pub const MINUS: u8 = 0x61;

// Jump Operations
pub const JUMP: u8 = 0x70;
pub const JUMP_IF_FALSE: u8 = 0x71;

// Global Variable Operations
pub const SET_GLOBAL: u8 = 0x80;
pub const GET_GLOBAL: u8 = 0x81;

// Local Variable Operations
pub const SET_LOCAL: u8 = 0x90;
pub const GET_LOCAL: u8 = 0x91;

// Builtin Function Operations
pub const GET_BUILTIN: u8 = 0xA0;

// Function Call Operations
pub const CALL: u8 = 0xB0;
pub const RETURN: u8 = 0xB1;
pub const RETURN_VALUE: u8 = 0xB2;

// Array Operations
pub const ARRAY: u8 = 0xC0;
pub const INDEX: u8 = 0xC1;

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
        assert_eq!(bytes[0], 32);
        assert_eq!(bytes[1], 255);
        assert_eq!(bytes[2], 254);
    }
}
