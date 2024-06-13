use crate::token;

#[derive(Debug, Clone)]
pub struct NullLiteral {
    pub token: token::Token,
}

impl std::fmt::Display for NullLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "null")
    }
}
