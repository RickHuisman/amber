use crate::syntax::ast::*;
use crate::syntax::token::*;
use crate::syntax::parser::{AstParser, ParserError};

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    None,
    Assign, // =
    Or,
    And,
    Equality,   // == !=
    Comparison, // < <= > >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // ()
    Primary,
}

impl<'a> From<&TokenType> for Precedence {
    fn from(token: &TokenType) -> Precedence {
        match token {
            TokenType::Equal => Precedence::Assign,
            // TokenType::Or => Precedence::Or, TODO
            // TokenType::And => Precedence::And, TODO
            TokenType::BangEqual | TokenType::EqualEqual => Precedence::Equality,
            TokenType::LessThan
            | TokenType::LessThanEqual
            | TokenType::GreaterThan
            | TokenType::GreaterThanEqual => Precedence::Comparison,
            TokenType::Plus | TokenType::Minus => Precedence::Term,
            TokenType::Star | TokenType::Slash => Precedence::Factor,
            TokenType::Bang => Precedence::Unary,
            TokenType::LeftParen => Precedence::Call,
            TokenType::Dot => Precedence::Call,
            _ => Precedence::None,
        }
    }
}

pub fn parse(parser: &mut AstParser) -> Result<Expr, ParserError> {
    parse_expr(parser, Precedence::None)
}

fn parse_expr(parser: &mut AstParser, precedence: Precedence) -> Result<Expr, ParserError> {
    let mut expr = parse_prefix(parser)?;
    while !parser.is_at_end() {
        let next_precedence = Precedence::from(parser.peek_type()?);
        if precedence >= next_precedence {
            break;
        }
        expr = parse_infix(parser, expr)?;
    }
    Ok(expr)
}

fn parse_prefix(parser: &mut AstParser) -> Result<Expr, ParserError> {
    match parser.peek_type()? {
        TokenType::Number
        // TODO | TokenType::Nil
        // TODO | TokenType::This
        // TODO | TokenType::True
        // TODO | TokenType::False
        | TokenType::Identifier
        | TokenType::String => parse_primary(parser),
        TokenType::Bang | TokenType::Minus => parse_unary(parser),
        TokenType::LeftParen => parse_grouping(parser),
        _ => todo!(),
        // _ => Err(SyntaxError::Unexpected(parser.peek_token().clone())), TODO
    }
}

fn parse_infix(parser: &mut AstParser, left: Expr) -> Result<Expr, ParserError> {
    match parser.peek_type()? {
        TokenType::BangEqual
        | TokenType::EqualEqual
        | TokenType::LessThan
        | TokenType::LessThanEqual
        | TokenType::GreaterThan
        | TokenType::GreaterThanEqual
        | TokenType::Plus
        | TokenType::Minus
        | TokenType::Star
        | TokenType::Slash => parse_binary(parser, left),
        // TokenType::Or | TokenType::And => parse_logical(parser, left), TODO
        // TokenType::Equal => parse_assign(parser, left), TODO
        // TokenType::LeftParen => parse_call(parser, left), TODO
        _ => todo!(),
        // _ => Err(SyntaxError::Unexpected(parser.peek_token().clone())),
    }
}

fn parse_grouping(parser: &mut AstParser) -> Result<Expr, ParserError> {
    let left_paren = parser.expect(TokenType::LeftParen)?;
    let expr = parse_expr(parser, Precedence::None)?;
    let right_paren = parser.expect(TokenType::RightParen)?;

    Ok(Expr::Grouping(GroupingExpr::new(Box::new(expr))))
}

fn parse_binary(parser: &mut AstParser, left: Expr) -> Result<Expr, ParserError> {
    let precedence = Precedence::from(parser.peek_type()?);
    let operator = BinaryOperator::from_token(parser.consume()?.token_type()).unwrap(); // TODO Unwrap
    let right = parse_expr(parser, precedence)?;
    Ok(Expr::Binary(BinaryExpr::new(operator, Box::new(left), Box::new(right))))
}

fn parse_primary(parser: &mut AstParser) -> Result<Expr, ParserError> {
    let token = parser.consume()?;
    match token.token_type() {
        TokenType::Keyword(Keyword::Nil) => Ok(Expr::Literal(LiteralExpr::Nil)),
        TokenType::Number => Ok(Expr::number(token.source().parse::<f64>().unwrap())),
        //_ => Err(ParserError::ExpectedPrimary(tc.clone())), TODO
        _ => todo!(),
    }
}

fn parse_unary(parser: &mut AstParser) -> Result<Expr, ParserError> {
    let operator = UnaryOperator::from_token(parser.consume()?.token_type()).unwrap(); // TODO Unwrap
    let right = parse_expr(parser, Precedence::Unary)?;
    Ok(Expr::Unary(UnaryExpr::new(operator, Box::new(right))))
}
