pub const CONSTANT: u8 = 0x00;
pub const POP: u8 = 0x01;
pub const ADD: u8 = 0x02;
pub const SUB: u8 = 0x03;
pub const MUL: u8 = 0x04;
pub const DIV: u8 = 0x05;
pub const MOD: u8 = 0x06;
pub const TRUE: u8 = 0x07;
pub const FALSE: u8 = 0x08;
pub const EQ: u8 = 0x0A;
pub const NE: u8 = 0x0B;
pub const LT: u8 = 0x0C;
pub const LE: u8 = 0x0D;
pub const GT: u8 = 0x0E;
pub const GE: u8 = 0x0F;
pub const BANG: u8 = 0x10;
pub const MINUS: u8 = 0x11;
pub const JUMP: u8 = 0x12;
pub const JUMP_IF_FALSE: u8 = 0x13;
pub const NULL: u8 = 0x14;

pub fn constant(v: u16) -> [u8; 3] {
    [CONSTANT, (v >> 8) as u8, (v & 0xFF) as u8]
}

pub fn jump(v: u16) -> [u8; 3] {
    [JUMP, (v >> 8) as u8, (v & 0xFF) as u8]
}

pub fn jump_if_false(v: u16) -> [u8; 3] {
    [JUMP_IF_FALSE, (v >> 8) as u8, (v & 0xFF) as u8]
}

#[cfg(test)]
mod tests {
    use crate::code;

    #[test]
    fn constant() {
        let bytes = code::constant(65534);

        assert_eq!(bytes.len(), 3);
        assert_eq!(bytes[0], 0);
        assert_eq!(bytes[1], 255);
        assert_eq!(bytes[2], 254);
    }
}
