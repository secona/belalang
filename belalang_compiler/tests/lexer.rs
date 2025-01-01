#[macro_use]
mod common;

use belalang_compiler::tokens::Token;
use belalang_compiler::tokens::Lexer;

fn test_tokens(input: &str, tokens: Vec<Token>) {
    let mut lexer = Lexer::new(input.as_bytes());

    for expected in tokens {
        let tok = lexer.next_token().unwrap();
        assert_eq!(tok, expected);
    }
}


#[test]
fn all_tokens() {
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
fn string_tokens() {
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
fn integer_tokens() {
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
fn identifier_tokens() {
    test_tokens(
        "x; x + y",
        vec![
            Token::Ident("x".into()),
            Token::Semicolon,
            Token::Ident("x".into()),
            Token::Add,
            Token::Ident("y".into()),
        ]
    )
}

#[test]
fn escape_strings_tokens() {
    test_tokens(r#""\n""#, vec![Token::String("\n".into())]);
    test_tokens(r#""\r""#, vec![Token::String("\r".into())]);
    test_tokens(r#""\t""#, vec![Token::String("\t".into())]);
    test_tokens(r#""\"""#, vec![Token::String("\"".into())]);

    test_tokens(r#""\x0A""#, vec![Token::String("\n".into())]);
    test_tokens(r#""\x41""#, vec![Token::String("A".into())]);
}
