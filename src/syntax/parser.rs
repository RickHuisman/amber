use crate::syntax::ast::*;
use crate::syntax::token::{Token, Keyword, TokenType};

// TODO Move to error.rs
#[derive(Debug)]
pub enum ParserError {
    UnexpectedEOF,
    Expect(TokenType, TokenType, usize), // TODO It's posssible to name enum kind variables?
}

type Result<T> = std::result::Result<T, ParserError>;

pub struct AstParser<'a> {
    tokens: &'a mut Vec<Token<'a>>,
}

impl<'a> AstParser<'a> {
    fn new(tokens: &'a mut Vec<Token<'a>>) -> Self {
        tokens.reverse();
        AstParser { tokens }
    }

    pub fn parse(tokens: &'a mut Vec<Token<'a>>) -> Result<ModuleAst> {
        let mut parser = AstParser::new(tokens);

        let mut exprs = vec![];

        while !(parser.is_eof()?) {
            exprs.push(parser.parse_top_level_expr()?);
        }

        Ok(ModuleAst::new(exprs))
    }

    fn parse_top_level_expr(&mut self) -> Result<Expr> {
        match self.peek_type()? {
            TokenType::Keyword(Keyword::Let) => self.declare_let(),
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_expression_statement(&mut self) -> Result<Expr> {
        let expr = self.expression()?;
        self.expect(TokenType::Line)?;
        Ok(expr)
    }

    fn declare_let(&mut self) -> Result<Expr> {
        // Consume "let".
        self.consume()?;

        let ident = self.expect(TokenType::Identifier)?;
        let var = Variable::new(ident.source().to_string());

        let initializer = if self.match_(&TokenType::Equal)? {
            self.parse_expression_statement()?
        } else {
            self.expect(TokenType::Line)?;
            Expr::nil()
        };

        Ok(Expr::let_assign(LetAssignExpr::new(var, Box::new(initializer))))
    }

    fn expression(&mut self) -> Result<Expr> {
        super::expr_parser::parse(self)
    }

    pub fn expect(&mut self, expect: TokenType) -> Result<Token<'a>> {
        if self.check(&expect)? {
            Ok(self.consume()?)
        } else {
            Err(ParserError::Expect(
                expect,
                self.peek_type()?.clone(), // TODO Clone
                self.peek().unwrap().position().line().clone(), // TODO Clone
            ))
        }
    }

    fn match_(&mut self, token_type: &TokenType) -> Result<bool> {
        if !self.check(token_type)? {
            return Ok(false);
        }

        self.consume()?;
        Ok(true)
    }

    fn check(&self, token_type: &TokenType) -> Result<bool> {
        Ok(self.peek_type()? == token_type)
    }

    pub fn consume(&mut self) -> Result<Token<'a>> {
        self.tokens.pop().ok_or(ParserError::UnexpectedEOF)
    }

    pub fn peek_type(&self) -> Result<&TokenType> {
        Ok(self.peek()?.token_type())
    }

    fn peek(&self) -> Result<&Token<'a>> {
        self.tokens.last().ok_or(ParserError::UnexpectedEOF)
    }

    pub fn is_eof(&self) -> Result<bool> {
        Ok(self.check(&TokenType::EOF)?)
    }
}
