use crate::token;

#[derive(Debug, Clone)]
pub struct BooleanExpression {
    pub token: token::Token,
    pub value: bool,
}

impl std::fmt::Display for BooleanExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "BooleanExpression(value={})",
            &self.value.to_string()
        ))
    }
}
