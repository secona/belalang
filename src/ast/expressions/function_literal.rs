use crate::{ast, token};

#[derive(Debug, Clone)]
pub struct FunctionLiteral {
    pub token: token::Token,
    pub params: Vec<ast::Identifier>,
    pub body: ast::BlockStatement,
}

impl std::fmt::Display for FunctionLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = self
            .params
            .iter()
            .map(|param| param.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "fn({}) {}", params, self.body)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, testing, token};

    #[test]
    fn parsing() {
        let program = testing::test_parse("fn(x, y) { x + y; }");

        assert_eq!(program.statements.len(), 1);

        let stmt =
            testing::as_variant!(&program.statements[0], ast::Statement::ExpressionStatement);

        let function = testing::as_variant!(&stmt.expression, ast::Expression::FunctionLiteral);

        assert_eq!(function.params.len(), 2);

        testing::ident_has_name!(function.params[0], "x");
        testing::ident_has_name!(function.params[1], "y");

        assert_eq!(function.body.statements.len(), 1);

        let body_stmt = testing::as_variant!(
            &function.body.statements[0],
            ast::Statement::ExpressionStatement
        );

        testing::expr_variant!(
            &body_stmt.expression, Infix => (
                ast::Expression::Identifier = "x",
                token::Token::Plus,
                ast::Expression::Identifier = "y"
            )
        );
    }

    #[test]
    fn parsing_parameters() {
        let tests: [(&str, Vec<&str>); 4] = [
            ("fn() {}", [].into()),
            ("fn(x) {};", ["x"].into()),
            ("fn(x, y) {};", ["x", "y"].into()),
            ("fn(x, y, z) {};", ["x", "y", "z"].into()),
        ];

        for test in tests {
            let program = testing::test_parse(test.0);

            let stmt =
                testing::as_variant!(&program.statements[0], ast::Statement::ExpressionStatement);

            let function = testing::as_variant!(&stmt.expression, ast::Expression::FunctionLiteral);

            for (i, exp) in test.1.iter().enumerate() {
                testing::ident_has_name!(function.params[i], *exp);
            }
        }
    }
}
