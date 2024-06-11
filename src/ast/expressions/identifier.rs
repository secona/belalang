use crate::token;

#[derive(Debug, Clone)]
pub struct Identifier {
    pub token: token::Token,
    pub value: String,
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, testing};

    #[test]
    fn parsing() {
        let program = testing::test_parse("name;");

        assert_eq!(program.statements.len(), 1);

        let expr =
            testing::as_variant!(&program.statements[0], ast::Statement::Expression);

        let ident = testing::as_variant!(&expr.expression, ast::Expression::Identifier);

        testing::ident_has_name!(ident, "name");
    }
}
