use crate::{error::TokenError, span::Span, token::Token, token_kind::TokenKind};

pub mod token_kind; 
pub mod span;
pub mod token;
pub mod error;




struct Lexer<'a> {
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

    fn next_token(&mut self) -> Result<Option<Token>, TokenError> { // also refered to as dispatcher
        // check for end of file

    





        unimplemented!()
    }
    
    fn ignore_whitespace() {unimplemented!()}
    

    
    // consume check ---{
    fn consume_if(&mut self, expected: char) -> bool { // used for clean one character tokens
        if self.peek() == Some(expected) {
            self.advance();
            true
        } else {
            false
        }
    }

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
    fn lex_ident_or_keyword(&mut self) -> Token {
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

        self.build_token(kind, lexeme, start)
    }

    fn lex_number(&mut self) -> Token {
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
        self.build_token(kind, lexeme, start)
    }

    fn lex_string() {}

    fn lex_char() {}
    
    fn symbol() {}


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

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token, TokenError>;

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