#[derive(PartialEq, Eq)]
pub enum Token {
    EMPTY,

    ILLEGAL(u8),
    EOF(u8),

    IDENT(u8),
    INT(u8),

    ASSIGN(u8),
    PLUS(u8),

    COMMA(u8),
    SEMICOLON(u8),

    LPAREN(u8),
    RPAREN(u8),
    LBRACE(u8),
    RBRACE(u8),

    FUNCTION(u8),
    LET(u8),
}
