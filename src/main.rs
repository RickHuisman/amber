use crate::syntax::lexer::Lexer;
use crate::syntax::parser::AstParser;

mod syntax;

fn main() {
    let source = r#"let x = 10
    let y = 20
    "#;

    let mut tokens = Lexer::tokenize(source).unwrap();
    let exprs = AstParser::parse(&mut tokens).unwrap();

    println!("{:?}", exprs);
}
