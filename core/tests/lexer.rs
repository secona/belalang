#[macro_use]
mod common;

use belalang_core::token::Token;
use common::test_tokens;

#[test]
fn tokens() {
    test_tokens(
        "=+(){}[],;!-/*5;5 < 10 > 5;:= >= <= += -= /= %= *= || &&",
        vec![
            Token::Assign,
            Token::Add,
            Token::LeftParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::RightBrace,
            Token::LeftBracket,
            Token::RightBracket,
            Token::Comma,
            Token::Semicolon,
            Token::Not,
            Token::Sub,
            Token::Div,
            Token::Mul,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Int("5".into()),
            Token::Lt,
            Token::Int("10".into()),
            Token::Gt,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::ColonAssign,
            Token::Ge,
            Token::Le,
            Token::AddAssign,
            Token::SubAssign,
            Token::DivAssign,
            Token::ModAssign,
            Token::MulAssign,
            Token::Or,
            Token::And,
        ],
    );
}

#[test]
fn strings_idents_nums() {
    test_tokens(
        r#""Hello, World";1230;x"#,
        vec![
            Token::String("Hello, World".into()),
            Token::Semicolon,
            Token::Int("1230".into()),
            Token::Semicolon,
            Token::Ident("x".into()),
        ],
    );
}

#[test]
fn escape_strings() {
    test_tokens(r#""\n""#, vec![Token::String("\n".into())]);
    test_tokens(r#""\r""#, vec![Token::String("\r".into())]);
    test_tokens(r#""\t""#, vec![Token::String("\t".into())]);
    test_tokens(r#""\"""#, vec![Token::String("\"".into())]);

    test_tokens(r#""\x0A""#, vec![Token::String("\n".into())]);
    test_tokens(r#""\x41""#, vec![Token::String("A".into())]);
}
