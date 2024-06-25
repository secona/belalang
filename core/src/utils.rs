macro_rules! unwrap_or_return {
    ($expr:expr, $ret:expr) => {
        match $expr {
            Some(v) => v,
            None => return $ret,
        }
    };
}

pub(super) use unwrap_or_return;

macro_rules! letters {
    () => {
        b'a'..=b'z' | b'A'..=b'Z' | b'_'
    };
}

pub(super) use letters;

macro_rules! digits {
    () => {
        b'0'..=b'9'
    };
}

pub(super) use digits;

pub fn hex_byte_to_u8(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
