use crate::{ast, token};

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

        f.write_str(&format!("fn({}) {}", params, self.body.to_string()))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{ast, lexer, parser, test_util};
//
//     #[test]
//     fn parsing() {
//         let input: Box<[u8]> = "fn(x, y) { x + y; }".as_bytes().into();
//
//         let lexer = lexer::Lexer::new(input);
//         let mut parser = parser::Parser::new(lexer);
//
//         let program = parser.parse_program().expect("got parser errors");
//
//         assert_eq!(program.statements.len(), 1);
//
//         let stmt = program.statements[0]
//             .downcast_ref::<ast::ExpressionStatement>()
//             .expect("not a(n) ast::ExpressionStatement");
//
//         let function = stmt
//             .expression
//             .downcast_ref::<ast::FunctionLiteral>()
//             .expect("not a(n) ast::FunctionLiteral");
//
//         assert_eq!(function.params.len(), 2);
//
//         test_util::test_identifier(function.params[0].as_ref(), "x");
//         test_util::test_identifier(function.params[1].as_ref(), "y");
//
//         assert_eq!(function.body.statements.len(), 1);
//
//         let body_stmt = function.body.statements[0]
//             .downcast_ref::<ast::ExpressionStatement>()
//             .expect("not a(n) ast::ExpressionStatement");
//
//         test_util::test_infix_expression(
//             body_stmt.expression.as_ref(),
//             test_util::Expected::Ident("x"),
//             "+",
//             test_util::Expected::Ident("y"),
//         );
//     }
//
//     #[test]
//     fn parameter_parsing() {
//         let tests: [(Box<[u8]>, Vec<&str>); 4] = [
//             ("fn() {}".as_bytes().into(), [].into()),
//             ("fn(x) {};".as_bytes().into(), ["x"].into()),
//             ("fn(x, y) {};".as_bytes().into(), ["x", "y"].into()),
//             ("fn(x, y, z) {};".as_bytes().into(), ["x", "y", "z"].into()),
//         ];
//
//         for test in tests {
//             let lexer = lexer::Lexer::new(test.0);
//             let mut parser = parser::Parser::new(lexer);
//
//             let program = parser.parse_program().expect("got parser errors");
//
//             let stmt = program.statements[0]
//                 .downcast_ref::<ast::ExpressionStatement>()
//                 .expect("not a(n) ast::ExpressionStatement");
//
//             let function = stmt
//                 .expression
//                 .downcast_ref::<ast::FunctionLiteral>()
//                 .expect("not a(n) ast::FunctionLiteral");
//
//             for (i, exp) in test.1.iter().enumerate() {
//                 test_util::test_identifier(function.params[i].as_ref(), exp);
//             }
//         }
//     }
// }
