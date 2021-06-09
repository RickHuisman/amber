use crate::syntax::ast::*;
use crate::syntax::token::Token;

// TODO Move to error.rs
#[derive(Debug)]
pub enum ParserError {}

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
        exprs.push(Expr::Var);

        Ok(ModuleAst::new(exprs))
    }

    fn parse_top_level_expr(&mut self) -> Result<Expr> {
        Ok(Expr::Var)
    }
}
