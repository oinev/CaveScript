use super::span::Span;
use super::token_kind::TokenKind;
use anyhow::{Ok, Result};

#[derive(Debug, Clone)]
pub struct Token<'src> {
    pub kind: TokenKind,
    pub lexeme: &'src str,
    pub span: Span,
}
// impl Token {
//     pub fn new(kind: TokenKind) -> Self {
//         Self {
//             kind,
//             span: Span::new(0, 0),
//         }
//     }
// } 
