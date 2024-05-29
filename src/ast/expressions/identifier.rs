use crate::token;

pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("Identifier(value={})", &self.value))
    }
}
