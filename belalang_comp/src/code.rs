use belalang_core::token::Token;

use crate::error::CompileError;

pub const CONSTANT: u8 = 0x00;
pub const POP: u8 = 0x01;
pub const ADD: u8 = 0x02;
pub const SUB: u8 = 0x03;
pub const MUL: u8 = 0x04;
pub const DIV: u8 = 0x05;
pub const MOD: u8 = 0x06;
pub const TRUE: u8 = 0x07;
pub const FALSE: u8 = 0x08;

pub fn constant(v: u16) -> [u8; 3] {
    [CONSTANT, (v >> 8) as u8, (v & 0xFF) as u8]
}

pub trait ToBytecode {
    fn to_bytecode(self) -> Result<u8, CompileError>;
}

impl ToBytecode for Token {
    fn to_bytecode(self) -> Result<u8, CompileError> {
        match self {
            Token::Add => Ok(ADD),
            Token::Sub => Ok(SUB),
            Token::Mul => Ok(MUL),
            Token::Div => Ok(DIV),
            Token::Mod => Ok(MOD),
            _ => return Err(CompileError::UnknownInfixOp(self)),
        }
    }
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
