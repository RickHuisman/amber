use crate::syntax::token::TokenType;

#[derive(PartialEq, Debug)] // TODO Custom Debug impl
pub struct ModuleAst {
    exprs: Vec<Expr>,
}

impl ModuleAst {
    pub fn new(exprs: Vec<Expr>) -> Self {
        ModuleAst { exprs }
    }
}

#[derive(PartialEq, Debug)] // TODO Custom Debug impl
pub enum Expr {
    Literal(LiteralExpr),
    Grouping(GroupingExpr),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    LetAssign(LetAssignExpr),
    LetGet(LetGetExpr),
    LetSet(LetSetExpr),
    Function(FunctionExpr),
    Block(BlockExpr),
    Return(ReturnExpr),
}

#[derive(PartialEq, Debug)] // TODO Custom Debug impl
pub enum LiteralExpr {
    Number(f64),
    Nil,
}

#[derive(PartialEq, Debug)]
pub struct GroupingExpr {
    expr: Box<Expr>,
}

impl GroupingExpr {
    pub fn new(expr: Box<Expr>) -> Self {
        GroupingExpr { expr }
    }
}

#[derive(PartialEq, Debug)]
pub enum BinaryOperator {
    Equal,
    BangEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Subtract,
    Add,
    Divide,
    Multiply,
}

impl BinaryOperator {
    pub fn from_token(token_type: &TokenType) -> Option<BinaryOperator> {
        let op = match token_type {
            TokenType::Minus => BinaryOperator::Subtract,
            TokenType::Plus => BinaryOperator::Add,
            TokenType::Star => BinaryOperator::Multiply,
            TokenType::Slash => BinaryOperator::Divide,
            TokenType::BangEqual => BinaryOperator::BangEqual,
            TokenType::Equal => BinaryOperator::Equal,
            TokenType::EqualEqual => BinaryOperator::Equal,
            TokenType::LessThan => BinaryOperator::LessThan,
            TokenType::LessThanEqual => BinaryOperator::LessThanEqual,
            TokenType::GreaterThan => BinaryOperator::GreaterThan,
            TokenType::GreaterThanEqual => BinaryOperator::GreaterThanEqual,
            _ => return None,
        };

        Some(op)
    }
}

#[derive(PartialEq, Debug)]
pub struct BinaryExpr {
    lhs: Box<Expr>,
    rhs: Box<Expr>,
    operator: BinaryOperator,
}

impl BinaryExpr {
    pub fn new(operator: BinaryOperator, lhs: Box<Expr>, rhs: Box<Expr>) -> BinaryExpr {
        BinaryExpr { operator, lhs, rhs }
    }
}

#[derive(PartialEq, Debug)]
pub enum UnaryOperator {
    Negate,
    Not,
}

impl UnaryOperator {
    pub fn from_token(token_type: &TokenType) -> Option<UnaryOperator> {
        Some(match token_type {
            TokenType::Minus => UnaryOperator::Negate,
            TokenType::Bang => UnaryOperator::Not,
            _ => return None,
        })
    }
}

#[derive(PartialEq, Debug)]
pub struct UnaryExpr {
    operator: UnaryOperator,
    expr: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(operator: UnaryOperator, expr: Box<Expr>) -> UnaryExpr {
        UnaryExpr { operator, expr }
    }
}

#[derive(PartialEq, Debug)]
pub struct LetAssignExpr {
    pub variable: Variable,
    pub initializer: Box<Expr>,
}

impl LetAssignExpr {
    pub fn new(variable: Variable, initializer: Box<Expr>) -> Self {
        LetAssignExpr {
            variable,
            initializer,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct LetGetExpr {
    pub variable: Variable,
}

impl LetGetExpr {
    pub fn new(variable: Variable) -> Self {
        LetGetExpr { variable }
    }
}

#[derive(PartialEq, Debug)]
pub struct LetSetExpr {
    pub variable: Variable,
    pub initializer: Box<Expr>,
}

impl LetSetExpr {
    pub fn new(variable: Variable, initializer: Box<Expr>) -> Self {
        LetSetExpr {
            variable,
            initializer,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct FunctionDeclaration {
    pub parameters: Vec<Variable>,
    pub body: BlockExpr,
}

impl FunctionDeclaration {
    pub fn new(parameters: Vec<Variable>, body: BlockExpr) -> Self {
        FunctionDeclaration { parameters, body }
    }
}

#[derive(PartialEq, Debug)]
pub struct FunctionExpr {
    pub variable: Variable,
    pub declaration: FunctionDeclaration,
}

impl FunctionExpr {
    pub fn new(variable: Variable, declaration: FunctionDeclaration) -> Self {
        FunctionExpr {
            variable,
            declaration,
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct BlockExpr {
    pub exprs: Vec<Expr>,
}

impl BlockExpr {
    pub fn new(exprs: Vec<Expr>) -> Self {
        BlockExpr { exprs }
    }
}

#[derive(PartialEq, Debug)]
pub struct ReturnExpr {
    pub expr: Option<Box<Expr>>,
}

impl ReturnExpr {
    pub fn new(expr: Option<Box<Expr>>) -> Self {
        ReturnExpr { expr }
    }
}

#[derive(PartialEq, Debug)]
pub struct Variable {
    pub name: String,
}

impl Variable {
    pub fn new(name: String) -> Self {
        Variable { name }
    }
}
