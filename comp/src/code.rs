#[repr(u8)]
pub enum Code {
    Constant(u16),
}

impl Code {
    fn discriminant(&self) -> u8 {
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl Into<Vec<u8>> for Code {
    fn into(self) -> Vec<u8> {
        let mut result = vec![self.discriminant()];

        match self {
            Self::Constant(v) => {
                result.push((v >> 8) as u8);
                result.push((v & 0xFF) as u8);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::Code;

    #[test]
    fn constant() {
        let code = Code::Constant(65534);
        let bytes: Vec<u8> = code.into();

        assert_eq!(bytes.len(), 3);
        assert_eq!(bytes[0], 0);
        assert_eq!(bytes[1], 255);
        assert_eq!(bytes[2], 254);
    }
}
