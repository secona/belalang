use crate::token;

pub struct IntegerLiteral {
    pub token: token::Token,
    pub value: i64,
}

impl std::fmt::Display for IntegerLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "IntegerLiteral(value={})",
            &self.value.to_string()
        ))
    }
}
