use super::span::Span;
use super::token_kind::TokenKind;

#[derive(Debug, Clone)]
pub struct Token<'src> {
    pub kind: TokenKind,
    pub lexeme: &'src str,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    // Special

    // Literals
    Identifier,
    Integer,
    Float,
    String,
    Char,
    True,
    False,

    // Primitive types
    IntTy,
    FloatTy,
    BoolTy,
    CharTy,
    StringTy,

    // Keywords
    Let,
    Var,
    Fn,
    Return,
    Type,
    Struct,
    Enum,
    Impl,
    To,
    //If,
    //Else,
    //Match,
    //While,
    //For,
    SelfKw,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,

    EqualEqual,
    BangEqual,

    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    AndAnd,
    OrOr,

    Bang, // !

    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Colon,
    ColonColon,
    Comma,
    Dot,
}