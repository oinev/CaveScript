
pub enum TokenError {
    Error, // ideally never use this


    // lex string
    UnterminatedString,


    // lex Char
    UnterminatedChar,
    UnterminatedEscape,
    InvalidEscape(char),
    InvalidCharLiteral,
    EmptyCharLiteral,

    // symbols
    UnexpectedCharacter(char),
}