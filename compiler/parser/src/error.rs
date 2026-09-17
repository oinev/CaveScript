use lexer::token::TokenKind;

pub enum ParseError {
    UnexpectedToken{
        expected: TokenKind,
        found: TokenKind,
    },
    UnexpectedEOF,
    UnexpectedNextTokenEOF
}