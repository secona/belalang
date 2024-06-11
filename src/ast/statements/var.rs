use crate::{
    ast::{Expression, Identifier},
    token,
};

#[derive(Debug, Clone)]
pub struct Var {
    pub token: token::Token,
    pub name: Identifier,
    pub value: Expression,
}

impl std::fmt::Display for Var {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {};", self.name, self.token, self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, testing, token};

    #[test]
    fn parsing_walrus() {
        let program = testing::test_parse("x := 5;");

        assert_eq!(program.statements.len(), 1);

        let stmt = testing::as_variant!(&program.statements[0], ast::Statement::Var);

        assert_eq!(stmt.token, token::Token::Walrus);
        testing::ident_has_name!(stmt.name, "x");

        testing::expr_variant!(&stmt.value, ast::Expression::IntegerLiteral = 5);
    }

    #[test]
    fn parsing_assign() {
        let program = testing::test_parse("x = 5;");

        assert_eq!(program.statements.len(), 1);

        let stmt = testing::as_variant!(&program.statements[0], ast::Statement::Var);

        assert_eq!(stmt.token, token::Token::Assign);
        testing::ident_has_name!(stmt.name, "x");

        testing::expr_variant!(&stmt.value, ast::Expression::IntegerLiteral = 5);
    }
}
