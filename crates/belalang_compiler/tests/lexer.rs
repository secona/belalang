#[macro_use]
mod common;

use belalang_compiler::tokens::Lexer;
use belalang_compiler::tokens::Token;

fn test_tokens(input: &str, expected: Vec<Token>) {
    let mut lexer = Lexer::new(input);
    let mut result = Vec::new();
    while let Ok(token) = lexer.next_token() {
        if let Token::EOF = token {
            break;
        }
        result.push(token);
    }
    assert_eq!(result, expected);
}

#[test]
fn tokens_all() {
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
fn tokens_strings() {
    test_tokens(
        r#""Hello, World"; "Test""#,
        vec![
            Token::String("Hello, World".into()),
            Token::Semicolon,
            Token::String("Test".into()),
        ],
    );
}

#[test]
fn tokens_integers() {
    test_tokens(
        "123; 456; 789 + 1",
        vec![
            Token::Int("123".into()),
            Token::Semicolon,
            Token::Int("456".into()),
            Token::Semicolon,
            Token::Int("789".into()),
            Token::Add,
            Token::Int("1".into()),
        ],
    );
}

#[test]
fn tokens_identifiers() {
    test_tokens(
        "x; x + y",
        vec![
            Token::Ident("x".into()),
            Token::Semicolon,
            Token::Ident("x".into()),
            Token::Add,
            Token::Ident("y".into()),
        ],
    );
}

#[test]
fn tokens_escape_strings() {
    test_tokens(r#""\n\r\t\"\x41""#, vec![Token::String("\n\r\t\"A".into())]);
}
