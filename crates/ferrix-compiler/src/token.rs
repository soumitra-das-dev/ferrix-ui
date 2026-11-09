//! Lexical token definitions and source spans for the FSX compiler.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub col: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, col: usize) -> Self {
        Self {
            start,
            end,
            line,
            col,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    // Keywords
    Import,
    From,
    Type,
    Export,
    Function,
    Return,
    Const,
    Let,
    True,
    False,

    // Identifiers & Literals
    Ident(String),
    Int(i64),
    Float(f64),
    StringLit(String),
    TextContent(String),

    // Delimiters & Punctuation
    OpenBrace,    // {
    CloseBrace,   // }
    OpenParen,    // (
    CloseParen,   // )
    OpenBracket,  // [
    CloseBracket, // ]
    Less,         // <
    Greater,      // >
    LessSlash,    // </
    SlashGreater, // />
    Colon,        // :
    Semi,         // ;
    Comma,        // ,
    Dot,          // .
    Equal,        // =
    FatArrow,     // =>

    // Operators
    Plus,  // +
    Minus, // -
    Star,  // *
    Slash, // /
    Bang,  // !

    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}
