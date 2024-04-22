use crate::{ast::{Expression, Node}, token};

pub struct CallExpression {
    pub token: token::Token,
    pub function: Box<dyn Expression>,
    pub args: Vec<Box<dyn Expression>>,
}

impl ToString for CallExpression {
    fn to_string(&self) -> String {
        format!(
            "{}({})",
            self.function.to_string(),
            self.args
                .iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}

impl Node for CallExpression {
    fn token(&self) -> Option<&token::Token> {
        Some(&self.token)
    }
}

impl Expression for CallExpression {
    fn expression_node(&self) {}
}

#[cfg(test)]
mod tests {
    use crate::{ast, lexer, parser, test_util};

    #[test]
    fn parsing() {
        let input = "add(1, 2 * 3, 4 + 5)".as_bytes().into();

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program().expect("got parser errors");

        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements[0]
            .downcast_ref::<ast::ExpressionStatement>()
            .expect("not a(n) ast::ExpressionStatement");

        let expr = stmt.expression
            .downcast_ref::<ast::CallExpression>()
            .expect("not a(n) ast::CallExpression");

        test_util::test_identifier(expr.function.as_ref(), "add");

        assert_eq!(expr.args.len(), 3);
        test_util::test_integer_literal(expr.args[0].as_ref(), 1);
        test_util::test_infix_expression(
            expr.args[1].as_ref(),
            test_util::Expected::Integer(2),
            "*",
            test_util::Expected::Integer(3),
        );
        test_util::test_infix_expression(
            expr.args[2].as_ref(),
            test_util::Expected::Integer(4),
            "+",
            test_util::Expected::Integer(5),
        );
    }

    #[test]
    fn parsing_function_literal() {
        let input = "fn(x, y) { x + y }(2, 3)".as_bytes().into();

        let lexer = lexer::Lexer::new(input);
        let mut parser = parser::Parser::new(lexer);
        let program = parser.parse_program().expect("got parser errors");

        assert_eq!(program.statements.len(), 1);

        let stmt = program.statements[0]
            .downcast_ref::<ast::ExpressionStatement>()
            .expect("not a(n) ast::ExpressionStatement");

        let expr = stmt
            .expression
            .downcast_ref::<ast::CallExpression>()
            .expect("not a(n) ast::CallExpression");

        assert_eq!(expr.args.len(), 2);
        test_util::test_integer_literal(expr.args[0].as_ref(), 2);
        test_util::test_integer_literal(expr.args[1].as_ref(), 3);

        let function = expr.function
            .downcast_ref::<ast::FunctionLiteral>()
            .expect("not a(n) ast::FunctionLiteral");

        assert_eq!(function.params.len(), 2);

        test_util::test_identifier(function.params[0].as_ref(), "x");
        test_util::test_identifier(function.params[1].as_ref(), "y");

        assert_eq!(function.body.statements.len(), 1);

        let body_stmt = function.body.statements[0]
            .downcast_ref::<ast::ExpressionStatement>()
            .expect("not a(n) ast::ExpressionStatement");

        test_util::test_infix_expression(
            body_stmt.expression.as_ref(),
            test_util::Expected::Ident("x"),
            "+",
            test_util::Expected::Ident("y"),
        );
    }
}
