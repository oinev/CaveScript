mod ast;

use lexer::{Lexer, error::TokenError, token::{Token, TokenKind}};
use crate::ast::Module;


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

    fn peek_kind(&self) -> Option<TokenKind> {
        Some(self.peek()?.kind)
    }

    fn advance(&mut self) -> Option<&Token<'a>> {
        let token = self.tokens.get(self.cursor)?;
        self.cursor += 1;
        Some(token)
    }
    fn parse(&self) {
        // check for end of file
        if self.peek_next() == None { return;}
        
    }
}