#[macro_use]
mod common;

use belalang::token::Token;
use common::test_tokens;

#[test]
fn tokens() {
    test_tokens(
        "=+(){},;!-/*5;5 < 10 > 5;:=",
        vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Int("5".into()),
            Token::LT,
            Token::Int("10".into()),
            Token::GT,
            Token::Int("5".into()),
            Token::Semicolon,
            Token::Walrus,
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
