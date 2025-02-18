#[macro_use]
mod common;

use belalang_compiler::tokens::Lexer;
use belalang_compiler::tokens::Token;

use test_case::test_case;

#[test_case(
    "=+(){}[],;!-/*5;5 < 10 > 5;:= >= <= += -= /= %= *= || &&" => vec![
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
    ]; "all"
)]
#[test_case(
    r#""Hello, World"; "Test""# => vec![
        Token::String("Hello, World".into()),
        Token::Semicolon,
        Token::String("Test".into()),
    ]; "strings"
)]
#[test_case(
    "123; 456; 789 + 1" => vec![
        Token::Int("123".into()),
        Token::Semicolon,
        Token::Int("456".into()),
        Token::Semicolon,
        Token::Int("789".into()),
        Token::Add,
        Token::Int("1".into()),
    ]; "integers"
)]
#[test_case(
    "x; x + y" => vec![
        Token::Ident("x".into()),
        Token::Semicolon,
        Token::Ident("x".into()),
        Token::Add,
        Token::Ident("y".into()),
    ]; "identifiers"
)]
#[test_case(
    r#""\n\r\t\"\x41""# => vec![
        Token::String("\n\r\t\"A".into()),
    ]; "escape_strings"
)]
fn tokens(input: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(input.as_bytes());
    let mut result = Vec::new();

    while let Ok(token) = lexer.next_token() {
        if let Token::EOF = token {
            break;
        }

        result.push(token);
    }

    result
}
