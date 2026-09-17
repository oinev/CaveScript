mod ast;
mod error;

use lexer::{Lexer, error::TokenError, token::{Token, TokenKind}};
use crate::error::ParseError;
use crate::ast::{Module, statement::*, expression::*, declaration::*, type_syntax::*};


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

    // utils

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

    fn check(&self, kind: TokenKind) -> bool {
        self.peek_kind() == Some(kind)
    }

    fn consume(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<&Token<'a>, ParseError> {
        match self.peek() {
            Some(token) if token.kind == kind => {
                self.advance().ok_or(ParseError::UnexpectedNextTokenEOF)
            }
            Some(token) => Err(ParseError::UnexpectedToken {
                expected: kind,
                found: token.kind,
            }),
            None => Err(ParseError::UnexpectedEOF),
        }
    } 

    fn parse(&self) {  // 
        // check for end of file
        if self.peek_next() == None { return;}

        
        
    }
