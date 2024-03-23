#[derive(PartialEq, Eq, Debug)]
pub enum Token {
    EMPTY,

    ILLEGAL(&'static u8),
    EOF(),

    IDENT(&'static [u8]),
    INT(&'static [u8]),

    ASSIGN(&'static u8),
    PLUS(&'static u8),
    MINUS(&'static u8),
    BANG(&'static u8),
    ASTERISK(&'static u8),
    SLASH(&'static u8),

    LT(&'static u8),
    GT(&'static u8),
    EQ(&'static [u8]),
    NOTEQ(&'static [u8]),

    COMMA(&'static u8),
    SEMICOLON(&'static u8),

    LPAREN(&'static u8),
    RPAREN(&'static u8),
    LBRACE(&'static u8),
    RBRACE(&'static u8),

    FUNCTION(&'static [u8]),
    LET(&'static [u8]),
    TRUE(&'static [u8]),
    FALSE(&'static [u8]),
    IF(&'static [u8]),
    ELSE(&'static [u8]),
    RETURN(&'static [u8]),
}

impl Token {
    pub fn lookup_ident(ident: &'static [u8]) -> Token {
        match ident {
            b"let" => Token::LET(ident),
            b"fn" => Token::FUNCTION(ident),
            b"true" => Token::TRUE(ident),
            b"false" => Token::FALSE(ident),
            b"if" => Token::IF(ident),
            b"else" => Token::ELSE(ident),
            b"return" => Token::RETURN(ident),
            _ => Token::IDENT(ident),
        }
    }
}
