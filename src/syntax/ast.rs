#[derive(Debug)] // TODO Custom Debug impl
pub struct ModuleAst {
    exprs: Vec<Expr>,
}

impl ModuleAst {
    pub fn new(exprs: Vec<Expr>) -> Self {
        ModuleAst { exprs }
    }
}

#[derive(Debug)] // TODO Custom Debug impl
pub enum Expr {
    Var,
}
