use std::iter::Peekable;
use std::str::CharIndices;

pub enum SyntaxError {
    UnexpectedEOF,
}

struct Position {
    start: usize,
    end: usize,
    line: usize,
}

struct Token<'a> {
    token_type: TokenType,
    source: &'a str,
    position: Position,
}

enum TokenType {
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

pub enum Keyword {
}

type Result<T> = std::result::Result<T, SyntaxError>;

struct Lexer<'a> {
    source: &'a str,
    chars: Peekable<CharIndices<'a>>,
    line: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Lexer {
            source,
            chars: source.char_indices().peekable(),
            line: 1,
        }
    }

    pub fn tokenize(source: &str) -> Result<Vec<Token>> {
        let mut lexer = Lexer::new(source);

        let mut tokens = vec![];
        while !lexer.is_at_end() {
            tokens.push(lexer.read_token()?);
        }

        Ok(tokens)
    }

    fn read_token(&mut self) -> Result<Token<'a>> {
        let (start, c) = self.advance().ok_or(SyntaxError::UnexpectedEOF)?;

        let token_type = match c {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            _ => todo!(),
        };

        Ok(self.make_token(start, token_type))
    }

    fn make_token(&mut self, start: usize, token_type: TokenType) -> Token<'a> {
        let end = start + self.source.len();
        let source = &self.source[start..end];
        let position = Position {
            start,
            end: start + self.source.len(),
            line: self.line
        };
        Token { token_type, source, position }
    }

    fn advance(&mut self) -> Option<(usize, char)> {
        self.chars.next().map(|(current, c)| {
            if c == '\n' {
                self.line += 1;
            }
            (current, c)
        })
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|&(_, c)| c)
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().is_none()
    }
}

fn main() {
    // let source = r#"let x = 10"#;
    let source = r#"("#;
    let tokens = Lexer::tokenize(source);
    match tokens {
        Ok(t) => {
            for token in t {
                println!("{:?}", token.source);
            }
        },
        _ => todo!(),
    }
}
