#[derive(PartialEq, Eq, Debug)]
pub enum Token<'a> {
    EMPTY,

    ILLEGAL(&'a u8),
    EOF(),

    IDENT(&'a [u8]),
    INT(&'a [u8]),

    ASSIGN(&'a u8),
    PLUS(&'a u8),
    MINUS(&'a u8),
    BANG(&'a u8),
    ASTERISK(&'a u8),
    SLASH(&'a u8),

    LT(&'a u8),
    GT(&'a u8),
    EQ(&'a [u8]),
    NOTEQ(&'a [u8]),

    COMMA(&'a u8),
    SEMICOLON(&'a u8),

    LPAREN(&'a u8),
    RPAREN(&'a u8),
    LBRACE(&'a u8),
    RBRACE(&'a u8),

    FUNCTION(&'a [u8]),
    LET(&'a [u8]),
    TRUE(&'a [u8]),
    FALSE(&'a [u8]),
    IF(&'a [u8]),
    ELSE(&'a [u8]),
    RETURN(&'a [u8]),
}

impl<'a> Token<'a> {
    pub fn lookup_ident(ident: &'a [u8]) -> Token {
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
