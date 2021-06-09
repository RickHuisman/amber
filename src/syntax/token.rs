use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    source: &'a str,
    position: Position,
}

impl<'a> Token<'a> {
    pub fn new(token_type: TokenType, source: &'a str, position: Position) -> Self {
        Token {
            token_type,
            source,
            position,
        }
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
    Slash,
    Semicolon,

    // Literals
    String,
    Number,

    // Keywords
    Keyword(Keyword),
    Identifier,

    EOF,
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Let,
    For,
    While,
}

impl FromStr for Keyword {
    type Err = ();

    fn from_str(source: &str) -> Result<Self, Self::Err> {
        Ok(match source {
            "let" => Keyword::Let,
            "for" => Keyword::For,
            "while" => Keyword::While,
            _ => return Err(()),
        })
    }
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
