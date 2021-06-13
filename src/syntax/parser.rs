use crate::syntax::ast::*;
use crate::syntax::token::{Keyword, Token, TokenType};

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
            TokenType::Keyword(Keyword::Def) => self.declare_def(),
            TokenType::Keyword(Keyword::Return) => self.parse_return(),
            _ => self.parse_expression_statement(),
        }
    }

    fn declare_let(&mut self) -> Result<Expr> {
        // Consume "let".
        self.expect(TokenType::Keyword(Keyword::Let))?;

        let ident = self.expect(TokenType::Identifier)?;
        let var = Variable::new(ident.source().to_string());

        let initializer = if self.match_(&TokenType::Equal)? {
            self.parse_expression_statement()?
        } else {
            self.expect(TokenType::Line)?;
            Expr::Literal(LiteralExpr::Nil)
        };

        Ok(Expr::LetAssign(LetAssignExpr::new(
            var,
            Box::new(initializer),
        )))
    }

    fn declare_def(&mut self) -> Result<Expr> {
        // Consume "def".
        self.expect(TokenType::Keyword(Keyword::Def))?;

        let ident = self.expect(TokenType::Identifier)?;
        let var = Variable::new(ident.source().to_string());

        self.expect(TokenType::LeftParen)?;

        let mut params = vec![];
        while !self.check(&TokenType::RightParen)? && !self.check(&TokenType::EOF)? {
            let param = self.expect(TokenType::Identifier)?;

            params.push(Variable::new(param.source().to_string())); // TODO clone?

            if !self.match_(&TokenType::Comma)? {
                break;
            }
        }

        self.expect(TokenType::RightParen)?;

        let body = match self.parse_block()? {
            // TODO
            Expr::Block(b) => b,
            _ => unreachable!(),
        };
        let fun_decl = FunctionDeclaration::new(params, body);

        Ok(Expr::Function(FunctionExpr::new(var, fun_decl)))
    }

    fn parse_return(&mut self) -> Result<Expr> {
        // Consume "return".
        self.expect(TokenType::Keyword(Keyword::Return))?;

        let return_expr = if self.match_(&TokenType::Line)? {
            // TODO
            None
        } else {
            Some(Box::new(self.parse_top_level_expr()?))
        };

        Ok(Expr::Return(ReturnExpr::new(return_expr)))
    }

    fn parse_block(&mut self) -> Result<Expr> {
        // self.consume()?; // Consume 'do' TODO

        self.match_(&TokenType::Line)?;

        let mut exprs = vec![];

        loop {
            if let TokenType::Keyword(Keyword::End) = self.peek_type()? {
                break;
            }

            // if self.check(TokenType::Keyword(Keyword::End))? ||
            //     self.check(TokenType::EOF)? {
            //     break;
            // }

            exprs.push(self.parse_top_level_expr()?);
        }

        self.expect(TokenType::Keyword(Keyword::End))?;
        self.expect(TokenType::Line)?;

        Ok(Expr::Block(BlockExpr::new(exprs)))
    }

    pub fn parse_expression_statement(&mut self) -> Result<Expr> {
        let expr = self.expression()?;
        self.expect(TokenType::Line)?;
        Ok(expr)
    }

    pub fn expression(&mut self) -> Result<Expr> {
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

    pub fn match_(&mut self, token_type: &TokenType) -> Result<bool> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::syntax::lexer::Lexer;

    #[test]
    fn parse_let_assign() {}

    #[test]
    fn parse_def() {
        let expected_exprs = vec![Expr::Function(FunctionExpr::new(
            Variable::new("double".to_string()),
            FunctionDeclaration::new(
                vec![Variable::new("x".to_string())],
                BlockExpr::new(vec![Expr::Return(ReturnExpr::new(Some(Box::new(
                    Expr::Binary(BinaryExpr::new(
                        BinaryOperator::Multiply,
                        Box::new(Expr::LetGet(LetGetExpr::new(Variable::new(
                            "x".to_string(),
                        )))),
                        Box::new(Expr::Literal(LiteralExpr::Number(2.0))),
                    )),
                ))))]),
            ),
        ))];
        let expect = ModuleAst::new(expected_exprs);

        let source = r#"def double(x)
            return x * 2
        end
        "#;
        let mut tokens = Lexer::tokenize(source).unwrap();
        let actual = AstParser::parse(&mut tokens).unwrap();

        assert_eq!(expect, actual);
    }
}
