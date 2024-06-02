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

        f.write_str(&format!(
            "FunctionLiteral(params=[{}], body=[{}])",
            params,
            self.body.to_string()
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::{ast, lexer, parser, testing};

    #[test]
    fn parsing() {
        let input: Box<[u8]> = "fn(x, y) { x + y; }".as_bytes().into();

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program().expect("got parser errors");

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
                "+",
                ast::Expression::Identifier = "y"
            )
        );
    }

    #[test]
    fn parameter_parsing() {
        let tests: [(Box<[u8]>, Vec<&str>); 4] = [
            ("fn() {}".as_bytes().into(), [].into()),
            ("fn(x) {};".as_bytes().into(), ["x"].into()),
            ("fn(x, y) {};".as_bytes().into(), ["x", "y"].into()),
            ("fn(x, y, z) {};".as_bytes().into(), ["x", "y", "z"].into()),
        ];

        for test in tests {
            let lexer = lexer::Lexer::new(test.0);
            let mut parser = parser::Parser::new(lexer);

            let program = parser.parse_program().expect("got parser errors");

            let stmt =
                testing::as_variant!(&program.statements[0], ast::Statement::ExpressionStatement);

            let function = testing::as_variant!(&stmt.expression, ast::Expression::FunctionLiteral);

            for (i, exp) in test.1.iter().enumerate() {
                testing::ident_has_name!(function.params[i], *exp);
            }
        }
    }
}
