#[derive(PartialEq, Eq)]
pub enum Token {
    EMPTY,

    ILLEGAL(&'static u8),
    EOF(),

    IDENT(&'static u8),
    INT(&'static u8),

    ASSIGN(&'static u8),
    PLUS(&'static u8),

    COMMA(&'static u8),
    SEMICOLON(&'static u8),

    LPAREN(&'static u8),
    RPAREN(&'static u8),
    LBRACE(&'static u8),
    RBRACE(&'static u8),

    FUNCTION(&'static u8),
    LET(&'static u8),
}
