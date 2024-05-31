pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer(i) => f.write_str(&format!("{}", i)),
            Self::Boolean(b) => f.write_str(&format!("{}", b)),
            Self::Null => f.write_str(&format!("null")),
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Integer(a), Object::Integer(b)) => a == b,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::Null, Object::Null) => true,
            _ => false,
        }
    }
}
