use std::iter::Peekable;
use std::str::CharIndices;
use crate::syntax::token::{Token, TokenType, Position};

// TODO Move to error.rs
#[derive(Debug, Clone)]
pub enum SyntaxError {
    UnexpectedEOF,
}

type Result<T> = std::result::Result<T, SyntaxError>;

pub struct Lexer<'a> {
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
        self.skip_whitespace();
        if self.is_at_end() {
            return self.eof();
        }

        let (start, c) = self.advance().ok_or(SyntaxError::UnexpectedEOF)?;

        if c.is_digit(10) {
            return self.number(start);
        }

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

        Ok(self.make_token(token_type, start))
    }

    fn number(&mut self, start: usize) -> Result<Token<'a>> {
        self.advance_while(|c| c.is_digit(10));

        // Look for a fractional part
        if let Some(peek) = self.peek() {
            if peek == '.' {
                if let Some(next) = self.peek_next() {
                    if next.is_digit(10) {
                        // Consume the '.'.
                        self.advance();

                        self.advance_while(|c| c.is_digit(10));
                    }
                }
            }
        }

        Ok(self.make_token(TokenType::Number, start))
    }

    fn eof(&mut self) -> Result<Token<'a>> {
        return Ok(self.make_token(TokenType::EOF, self.source.len()));
    }

    fn make_token(&mut self, token_type: TokenType, start: usize) -> Token<'a> {
        let source = self.token_contents(start);
        Token::new(
            token_type,
            source,
            Position::new(start, 10, self.line), // TODO
        )
    }

    fn token_contents(&mut self, start: usize) -> &'a str {
        let end = self
            .chars
            .peek()
            .map(|&(i, _)| i)
            .unwrap_or(self.source.len());
        &self.source[start..end].trim_end()
    }

    fn skip_whitespace(&mut self) {
        // TODO Increase line
        self.advance_while(|&c| c.is_whitespace());
    }

    fn advance_while<F>(&mut self, f: F) -> usize
    where
        for<'r> F: Fn(&'r char) -> bool,
    {
        let mut count = 0;
        while let Some(char) = self.peek() {
            if f(&char) {
                self.advance();
                count += 1;
            } else {
                break;
            }
        }
        count
    }

    fn advance(&mut self) -> Option<(usize, char)> {
        self.chars.next().map(|(current, c)| {
            if c == '\n' {
                self.line += 1;
            }
            (current, c)
        })
    }

    fn peek_next(&mut self) -> Option<char> {
        self.chars.nth(1).map(|(_, c)| c)
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.peek().map(|&(_, c)| c)
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_number() {
        let expect = vec![
            Token::new(TokenType::Number, "2", Position::new(0, 1, 1)),
            Token::new(TokenType::Number, "10", Position::new(0, 1, 1)),
            Token::new(TokenType::Number, "3.33", Position::new(0, 1, 1)),
            Token::new(TokenType::EOF, "", Position::new(0, 1, 1)),
        ];

        let source = r#"2
        10
        3.33
        "#;

        let actual = Lexer::tokenize(source).unwrap();
        assert_eq!(expect, actual);
    }
}
