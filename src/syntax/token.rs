#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    source: &'a str,
    position: Position,
}

impl<'a> Token<'a> {
    pub fn new(
        token_type: TokenType,
        source: &'a str,
        position: Position
    ) -> Self {
        Token { token_type, source, position }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Percent,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Arrow,
    Slash,
    Semicolon,
    Colon,

    // Literals
    String,
    Number,

    // Keywords
    Keyword(Keyword),
    Identifier,

    EOF,
}

#[derive(Debug, PartialEq)]
enum Keyword {
}

#[derive(Debug, PartialEq)]
pub struct Position {
    pub start: usize,
    pub end: usize,
    pub line: usize,
}

impl Position {
    pub fn new(start: usize, end: usize, line: usize) -> Self {
        Position { start, end, line }
    }
}
