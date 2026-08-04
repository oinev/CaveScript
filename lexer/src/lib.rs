
use crate::{error::TokenError, span::Span, token::Token, token::TokenKind};

pub mod span;
pub mod token;
pub mod error;




pub struct Lexer<'a> {
    source: &'a str,
    cursor: usize, // usize index into source / where you are in bytes from the start of file
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            cursor: 0,
        }
    }

    fn peek_n(&self, n: usize) -> Option<char> {
        debug_assert!(self.source.is_char_boundary(self.cursor));
        self.source[self.cursor..].chars().nth(n)
    }
    fn peek(&self) -> Option<char> {
        self.peek_n(0)
    }
    fn peek_next(&self) -> Option<char> {
        self.peek_n(1)
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.cursor += c.len_utf8();
        Some(c)
    }

    fn next_token(&mut self) -> Result<Option<Token<'a>>, TokenError> { // also refered to as dispatcher
        self.ignore_whitespaces();
        
        // check for end of file
        let token = match self.peek() { // checks for first character and maps to correct lexer
            None => return Ok(None),

            Some(c) if Self::is_identifier_start(c)
            => self.lex_ident_or_keyword(),

            Some(c) if c.is_ascii_digit()
            => self.lex_number(),

            Some('"') => self.lex_string(),

            Some('\'') => self.lex_char(),

            Some(_) => self.lex_symbol(),

        };

        return Ok(Some(token?))
    }
    
    fn ignore_whitespaces(&mut self) {
    while let Some(c) = self.peek() {
        if !c.is_whitespace() {
            break;
        }

        self.advance();
    }
}
    

    
    // consume check ---{
    fn consume_if(&mut self, expected: char) -> bool { // used for clean one character tokens
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    } // IGNORE FOR NOW, DELETE IF NOT USED

    fn consume_while<F>(&mut self, pred: F) -> &'a str
    where
    F: Fn(char) -> bool,
    {
        let start = self.cursor;

        while let Some(c) = self.peek() {
            if !pred(c) {
                break;
            }
            self.advance();
        }

        &self.source[start..self.cursor]
    }

    fn is_identifier_start(c: char) -> bool { // to use on dispacher
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_identifier_continue(c: char) -> bool { // to use on consumer, ex: foo123 is valid while 123foo is not
        c.is_ascii_alphanumeric() || c == '_'
    }
    // consume check ---}


    // lexers ---{
    fn lex_ident_or_keyword(&mut self) -> Result<Token<'a>, TokenError> {
        let start = self.cursor;

        let lexeme = self.consume_while(Self::is_identifier_continue);
        let kind = match lexeme {
            "let"    => TokenKind::Let,
            "var"    => TokenKind::Var,
            "return" => TokenKind::Return,
            "type"   => TokenKind::Type,
            "struct" => TokenKind::Struct,
            "enum"   => TokenKind::Enum,
            "impl"   => TokenKind::Impl,
            "to"     => TokenKind::To,
            "self"   => TokenKind::SelfKw,
            "fn"     => TokenKind::Fn,

            "true"   => TokenKind::True,
            "false"  => TokenKind::False,

            "int"    => TokenKind::IntTy,
            "float"  => TokenKind::FloatTy,
            "string" => TokenKind::StringTy,
            "char"   => TokenKind::CharTy,
            "bool"   => TokenKind::BoolTy,

            _ => TokenKind::Identifier
        };

        Ok(self.build_token(kind, lexeme, start))
    }

    fn lex_number(&mut self) -> Result<Token<'a>, TokenError> {
        let start = self.cursor;

        let lexeme = self.consume_while(|c| c.is_ascii_digit());
        let kind = if self.peek() == Some('.')
        && self.peek_next().is_some_and(|c| c.is_ascii_digit()) {
            self.advance(); // consume '.'
            self.consume_while(|c| c.is_ascii_digit());

            TokenKind::Float
        } else {
            TokenKind::Integer
        };
        Ok(self.build_token(kind, lexeme, start))
    }

    fn lex_string(&mut self) -> Result<Token<'a>, TokenError> {unimplemented!()}

    fn lex_char(&mut self) -> Result<Token<'a>, TokenError> {unimplemented!()}
    
    fn lex_symbol(&mut self) -> Result<Token<'a>, TokenError> {unimplemented!()}


    // helpers
    fn build_token<'b>(&self, kind: TokenKind, lexeme: &'b str, start: usize)  -> Token<'b> {
        Token {
            kind,
            lexeme,
            span: Span {
                start,
                end: self.cursor,
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, TokenError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(Some(token)) => Some(Ok(token)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}


// next()
//     ↓
// next_token()
//     ↓
// skip_whitespace()

//     ↓
// match on first character

// 'a'..='z' | 'A'..='Z' | '_'
//         ↓
// lex_identifier_or_keyword()

// '0'..='9'
//         ↓
// lex_number()

// '"'
//         ↓
// lex_string()

// '\''
//         ↓
// lex_char()

// everything else
//         ↓
// lex_symbol()