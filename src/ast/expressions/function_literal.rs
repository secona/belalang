use crate::{
    ast::{self, Expression, Node},
    token,
};

pub struct FunctionLiteral {
    pub token: token::Token,
    pub params: Vec<Box<ast::Identifier>>,
    pub body: ast::BlockStatement,
}

impl ToString for FunctionLiteral {
    fn to_string(&self) -> String {
        format!(
            "fn ({}) {}",
            self.params
                .iter()
                .map(|param| param.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.body.to_string(),
        )
    }
}

impl Node for FunctionLiteral {
    fn token(&self) -> Option<&token::Token> {
        Some(&self.token)
    }
}

impl Expression for FunctionLiteral {
    fn expression_node(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::{ast, lexer, parser, test_util};

    #[test]
    fn parsing() {
        let input: Box<[u8]> = "fn(x, y) { x + y; }".as_bytes().into();

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);

        let program = parser.parse_program();

        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements[0]
            .downcast_ref::<ast::ExpressionStatement>()
            .expect("not a(n) ast::ExpressionStatement");

        let function = stmt
            .expression
            .downcast_ref::<ast::FunctionLiteral>()
            .expect("not a(n) ast::FunctionLiteral");

        assert_eq!(function.params.len(), 2);

        test_util::test_identifier(function.params[0].as_ref(), "x".into());
        test_util::test_identifier(function.params[1].as_ref(), "y".into());

        assert_eq!(function.body.statements.len(), 1);

        let body_stmt = function.body.statements[0]
            .downcast_ref::<ast::ExpressionStatement>()
            .expect("not a(n) ast::ExpressionStatement");

        let infix_expr = body_stmt
            .expression
            .downcast_ref::<ast::InfixExpression>()
            .expect("not a(n) ast::InfixExpression");

        test_util::test_identifier(
            infix_expr
                .left
                .downcast_ref::<ast::Identifier>()
                .expect("not a(n) ast::Identifier"),
            "x".into(),
        );

        test_util::test_identifier(
            infix_expr
                .right
                .downcast_ref::<ast::Identifier>()
                .expect("not a(n) ast::Identifier"),
            "y".into(),
        );

        assert_eq!(infix_expr.operator, "+".to_owned());
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

            let program = parser.parse_program();

            let stmt = program.statements[0]
                .downcast_ref::<ast::ExpressionStatement>()
                .expect("not a(n) ast::ExpressionStatement");

            let function = stmt
                .expression
                .downcast_ref::<ast::FunctionLiteral>()
                .expect("not a(n) ast::FunctionLiteral");

            for (i, exp) in test.1.iter().enumerate() {
                test_util::test_identifier(function.params[i].as_ref(), exp.to_string());
            }
        }
    }
}
