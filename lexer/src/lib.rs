
use crate::{error::TokenError, span::Span, token::Token, token::TokenKind};

pub mod span;
pub mod token;
pub mod error;




pub struct Lexer<'a> {
    source: &'a str,
    cursor: usize, // usize index into source / where you are in bytes from the start of file
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
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
    fn consume_if(&mut self, expected: char) -> bool { // used for clean 1-2 character tokens
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

    fn is_valid_char_escape(c: char) -> bool {
        matches!(c, 'n' | 't' | 'r' | '\\' | '\'')
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

    fn lex_string(&mut self) -> Result<Token<'a>, TokenError> {
        let start = self.cursor;
        self.advance(); // consume "

        while let Some(c) = self.peek() {
            match c {
                '"' => break,
            
                '\\' => {
                    self.advance(); // consume /
                    self.advance(); // consume id, ex: /n ; /" ; etc
                }
            
                _ => {
                    self.advance(); // consume any character
                }
            }
        }
        if !self.consume_if('"') {
            return Err(TokenError::UnterminatedString)
        }
        let lexeme = &self.source[start..self.cursor];

        Ok(self.build_token(TokenKind::String, lexeme, start))

    }

    fn lex_char(&mut self) -> Result<Token<'a>, TokenError> {
        let start = self.cursor;

        self.advance(); // consume opening '

        match self.peek() {
            None => return Err(TokenError::UnterminatedChar),

            Some('\\') => {
                self.advance(); // consume '\'

                match self.advance() {
                    Some(c) if Self::is_valid_char_escape(c) => {}
                    Some(c) => return Err(TokenError::InvalidEscape(c)),
                    None => return Err(TokenError::UnterminatedEscape),   
                }
            }

            Some('\'') => return Err(TokenError::EmptyCharLiteral),

            Some(c) if c.is_ascii() => { self.advance(); }

            Some(_) => return Err(TokenError::InvalidCharLiteral),
        };

        if !self.consume_if('\'') {
            return Err(TokenError::UnterminatedChar);
        }
        let lexeme = &self.source[start..self.cursor];

        Ok(self.build_token(TokenKind::Char, lexeme, start))
    }
    
    fn lex_symbol(&mut self) -> Result<Token<'a>, TokenError> {
        let start = self.cursor;

        let kind = match self.advance() {

            // Operators 
            Some('+') => {
                if self.consume_if('=') {
                    TokenKind::PlusEqual
                } else {
                    TokenKind::Plus
                }
            },

            Some('-') => {
                if self.consume_if('=') {
                    TokenKind::MinusEqual
                } else {
                    TokenKind::Minus
                }
            },

            Some('*') => {
                if self.consume_if('=') {
                    TokenKind::StarEqual
                } else {
                    TokenKind::Star
                }
            },

            Some('/') => {
                if self.consume_if('=') {
                    TokenKind::SlashEqual
                } else {
                    TokenKind::Slash
                }
            },

            Some('%') => TokenKind::Percent, // hoping to implement %= later

            Some('!') => {
                if self.consume_if('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }

            Some('=') => {
                if self.consume_if('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            },

            Some('>') => {
                if self.consume_if('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            },

            Some('<') => {
                if self.consume_if('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            },

            Some('&') => {
                if self.consume_if('&') {
                    TokenKind::AndAnd
                } else {
                    return Err(TokenError::UnexpectedCharacter('&'))
                }
            },

            Some('|') => {
                if self.consume_if('|') {
                    TokenKind::OrOr
                } else {
                    return Err(TokenError::UnexpectedCharacter('|'))
                }
            },

            // Delimiters
            Some('(') => TokenKind::LeftParen,
            Some(')') => TokenKind::RightParen,

            Some('{') => TokenKind::LeftBrace,
            Some('}') => TokenKind::RightBrace,

            Some('[') => TokenKind::LeftBracket,
            Some(']') => TokenKind::RightBracket,

            Some(',') => TokenKind::Comma,
            Some('.') => TokenKind::Dot,

            Some(c) => {
            return Err(TokenError::UnexpectedCharacter(c))
            }

            None => unreachable!(),
        };        

        let lexeme = &self.source[start..self.cursor];

        Ok(self.build_token(kind, lexeme, start))
    }

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