use lexer::{Lexer, error::TokenError, token::Token};





pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    cursor: usize,
}

impl<'a> Parser<'a> {
    pub fn new(&mut self, tokens: Lexer<'a>) -> Result<Self, TokenError> {
        Ok(Self {
            tokens: tokens.collect::<Result<_, _>>()?,
            cursor: 0,
        })
    }

    fn peek_n(&self, n: usize) -> Option<&Token<'a>> {
        debug_assert!(self.cursor <= self.tokens.len()); 
        self.tokens[self.cursor..].get(n)
    }
    fn peek(&self) -> Option<&Token<'a>> {
        self.peek_n(0)
    }
    fn peek_next(&self) -> Option<&Token<'a>> {
        self.peek_n(1)
    }
}