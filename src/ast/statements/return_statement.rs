use crate::ast::expressions::Expression;
use crate::token;

pub struct ReturnStatement {
    pub token: token::Token,
    pub return_value: Expression,
}

impl std::fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("return {};", self.return_value.to_string()))
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{ast, test_util, token};
//
//     use super::ReturnStatement;
//
//     #[test]
//     fn to_string() {
//         let tests = [
//             test_util::ToStringTest {
//                 obj: ReturnStatement {
//                     token: token::Token::Return,
//                     return_value: Box::new(ast::Identifier {
//                         token: token::Token::Ident(String::from("x")),
//                         value: String::from("x"),
//                     }),
//                 },
//                 exp: String::from("return x;"),
//             },
//             test_util::ToStringTest {
//                 obj: ReturnStatement {
//                     token: token::Token::Return,
//                     return_value: Box::new(ast::IntegerLiteral {
//                         token: token::Token::Int(String::from("5")),
//                         value: 5,
//                     }),
//                 },
//                 exp: String::from("return 5;"),
//             },
//         ];
//
//         tests.map(|t| t.test());
//     }
// }
