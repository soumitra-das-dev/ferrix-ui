//! Lexer for the FSX declarative language.

use crate::token::{Span, Token, TokenKind};
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum LexError {
    #[error("Unexpected character '{0}' at line {1}, col {2}")]
    UnexpectedChar(char, usize, usize),
    #[error("Unterminated string literal starting at line {0}, col {1}")]
    UnterminatedString(usize, usize),
    #[error("Invalid number literal at line {0}, col {1}")]
    InvalidNumber(usize, usize),
}

pub struct Lexer<'a> {
    source: &'a str,
    chars: Vec<(usize, char)>,
    cursor: usize,
    line: usize,
    col: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let chars: Vec<(usize, char)> = source.char_indices().collect();
        Self {
            source,
            chars,
            cursor: 0,
            line: 1,
            col: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.cursor).map(|&(_, c)| c)
    }

    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.cursor + 1).map(|&(_, c)| c)
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(&(_, c)) = self.chars.get(self.cursor) {
            self.cursor += 1;
            if c == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
            Some(c)
        } else {
            None
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else if c == '/' && self.peek_next() == Some('/') {
                // Line comment
                while let Some(ch) = self.peek() {
                    self.advance();
                    if ch == '\n' {
                        break;
                    }
                }
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace_and_comments();

        let start_line = self.line;
        let start_col = self.col;
        let start_offset = self
            .chars
            .get(self.cursor)
            .map(|&(idx, _)| idx)
            .unwrap_or(self.source.len());

        let c = match self.peek() {
            Some(c) => c,
            None => {
                return Ok(Token::new(
                    TokenKind::Eof,
                    Span::new(start_offset, start_offset, start_line, start_col),
                ))
            }
        };

        // Identifiers & Keywords
        if c.is_alphabetic() || c == '_' {
            let mut ident = String::new();
            while let Some(ch) = self.peek() {
                if ch.is_alphanumeric() || ch == '_' {
                    ident.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
            let end_offset = self
                .chars
                .get(self.cursor)
                .map(|&(idx, _)| idx)
                .unwrap_or(self.source.len());
            let span = Span::new(start_offset, end_offset, start_line, start_col);

            let kind = match ident.as_str() {
                "import" => TokenKind::Import,
                "from" => TokenKind::From,
                "type" => TokenKind::Type,
                "export" => TokenKind::Export,
                "function" => TokenKind::Function,
                "return" => TokenKind::Return,
                "const" => TokenKind::Const,
                "let" => TokenKind::Let,
                "true" => TokenKind::True,
                "false" => TokenKind::False,
                _ => TokenKind::Ident(ident),
            };

            return Ok(Token::new(kind, span));
        }

        // Numbers (Hexadecimal, Integer and Float)
        if c.is_ascii_digit() {
            if c == '0' && (self.peek_next() == Some('x') || self.peek_next() == Some('X')) {
                self.advance(); // consume '0'
                self.advance(); // consume 'x'
                let mut hex_str = String::new();
                while let Some(ch) = self.peek() {
                    if ch.is_ascii_hexdigit() {
                        hex_str.push(ch);
                        self.advance();
                    } else {
                        break;
                    }
                }
                let end_offset = self
                    .chars
                    .get(self.cursor)
                    .map(|&(idx, _)| idx)
                    .unwrap_or(self.source.len());
                let span = Span::new(start_offset, end_offset, start_line, start_col);
                let val = u64::from_str_radix(&hex_str, 16)
                    .map_err(|_| LexError::InvalidNumber(start_line, start_col))?
                    as i64;
                return Ok(Token::new(TokenKind::Int(val), span));
            }

            let mut num_str = String::new();
            let mut is_float = false;
            while let Some(ch) = self.peek() {
                if ch.is_ascii_digit() {
                    num_str.push(ch);
                    self.advance();
                } else if ch == '.'
                    && !is_float
                    && self
                        .peek_next()
                        .map(|n| n.is_ascii_digit())
                        .unwrap_or(false)
                {
                    is_float = true;
                    num_str.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }
            let end_offset = self
                .chars
                .get(self.cursor)
                .map(|&(idx, _)| idx)
                .unwrap_or(self.source.len());
            let span = Span::new(start_offset, end_offset, start_line, start_col);

            let kind = if is_float {
                let f = num_str
                    .parse::<f64>()
                    .map_err(|_| LexError::InvalidNumber(start_line, start_col))?;
                TokenKind::Float(f)
            } else {
                let i = num_str
                    .parse::<i64>()
                    .map_err(|_| LexError::InvalidNumber(start_line, start_col))?;
                TokenKind::Int(i)
            };

            return Ok(Token::new(kind, span));
        }

        // String literals
        if c == '"' {
            self.advance(); // consume opening quote
            let mut content = String::new();
            let mut closed = false;
            while let Some(ch) = self.advance() {
                if ch == '"' {
                    closed = true;
                    break;
                }
                if ch == '\\' {
                    if let Some(escaped) = self.advance() {
                        match escaped {
                            'n' => content.push('\n'),
                            't' => content.push('\t'),
                            'r' => content.push('\r'),
                            '\\' => content.push('\\'),
                            '"' => content.push('"'),
                            other => content.push(other),
                        }
                    }
                } else {
                    content.push(ch);
                }
            }

            if !closed {
                return Err(LexError::UnterminatedString(start_line, start_col));
            }

            let end_offset = self
                .chars
                .get(self.cursor)
                .map(|&(idx, _)| idx)
                .unwrap_or(self.source.len());
            return Ok(Token::new(
                TokenKind::StringLit(content),
                Span::new(start_offset, end_offset, start_line, start_col),
            ));
        }

        // Punctuation and multi-character operators
        self.advance();
        let kind = match c {
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            '(' => TokenKind::OpenParen,
            ')' => TokenKind::CloseParen,
            '[' => TokenKind::OpenBracket,
            ']' => TokenKind::CloseBracket,
            ':' => TokenKind::Colon,
            ';' => TokenKind::Semi,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => {
                if self.peek() == Some('>') {
                    self.advance();
                    TokenKind::SlashGreater
                } else {
                    TokenKind::Slash
                }
            }
            '=' => {
                if self.peek() == Some('>') {
                    self.advance();
                    TokenKind::FatArrow
                } else {
                    TokenKind::Equal
                }
            }
            '<' => {
                if self.peek() == Some('/') {
                    self.advance();
                    TokenKind::LessSlash
                } else {
                    TokenKind::Less
                }
            }
            '>' => TokenKind::Greater,
            other => return Err(LexError::UnexpectedChar(other, start_line, start_col)),
        };

        let end_offset = self
            .chars
            .get(self.cursor)
            .map(|&(idx, _)| idx)
            .unwrap_or(self.source.len());
        Ok(Token::new(
            kind,
            Span::new(start_offset, end_offset, start_line, start_col),
        ))
    }

    /// Special lexing mode when expecting raw text inside a JSX element body.
    pub fn next_jsx_child(&mut self) -> Result<Token, LexError> {
        let start_line = self.line;
        let start_col = self.col;
        let start_offset = self
            .chars
            .get(self.cursor)
            .map(|&(idx, _)| idx)
            .unwrap_or(self.source.len());

        let c = match self.peek() {
            Some(c) => c,
            None => {
                return Ok(Token::new(
                    TokenKind::Eof,
                    Span::new(start_offset, start_offset, start_line, start_col),
                ))
            }
        };

        if c == '<' {
            self.advance();
            if self.peek() == Some('/') {
                self.advance();
                let end_offset = self
                    .chars
                    .get(self.cursor)
                    .map(|&(idx, _)| idx)
                    .unwrap_or(self.source.len());
                return Ok(Token::new(
                    TokenKind::LessSlash,
                    Span::new(start_offset, end_offset, start_line, start_col),
                ));
            } else {
                let end_offset = self
                    .chars
                    .get(self.cursor)
                    .map(|&(idx, _)| idx)
                    .unwrap_or(self.source.len());
                return Ok(Token::new(
                    TokenKind::Less,
                    Span::new(start_offset, end_offset, start_line, start_col),
                ));
            }
        }

        if c == '{' {
            self.advance();
            let end_offset = self
                .chars
                .get(self.cursor)
                .map(|&(idx, _)| idx)
                .unwrap_or(self.source.len());
            return Ok(Token::new(
                TokenKind::OpenBrace,
                Span::new(start_offset, end_offset, start_line, start_col),
            ));
        }

        // Accumulate text content until next '<' or '{'
        let mut text = String::new();
        while let Some(ch) = self.peek() {
            if ch == '<' || ch == '{' {
                break;
            }
            text.push(ch);
            self.advance();
        }

        let end_offset = self
            .chars
            .get(self.cursor)
            .map(|&(idx, _)| idx)
            .unwrap_or(self.source.len());
        Ok(Token::new(
            TokenKind::TextContent(text),
            Span::new(start_offset, end_offset, start_line, start_col),
        ))
    }

    /// Tokenizes the entire source string into a vector of tokens.
    pub fn tokenize_all(mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            let is_eof = token.kind == TokenKind::Eof;
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        Ok(tokens)
    }
}
