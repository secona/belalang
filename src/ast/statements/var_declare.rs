use crate::{
    ast::{Expression, Identifier},
    token,
};

#[derive(Debug, Clone)]
pub struct VarDeclare {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Expression,
}

impl std::fmt::Display for VarDeclare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} := {};", self.name, self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, testing, token};

    #[test]
    fn parsing() {
        let program = testing::test_parse("x := 5;");

        assert_eq!(program.statements.len(), 1);

        let stmt = testing::as_variant!(&program.statements[0], ast::Statement::VarDeclare);

        assert_eq!(stmt.token, token::Token::Walrus);
        assert_eq!(stmt.name.token, token::Token::Ident(String::from("x")));
        assert_eq!(stmt.name.value, String::from("x"));

        testing::expr_variant!(&stmt.value, ast::Expression::IntegerLiteral = 5);
    }
}
