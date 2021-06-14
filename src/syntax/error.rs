use crate::syntax::token::TokenType;

#[derive(Debug, Clone)]
pub enum SyntaxError {
    UnterminatedString,
    UnexpectedEOF,
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedEOF,
    Expect(TokenType, TokenType, usize), // TODO It's posssible to name enum kind variables?
}
