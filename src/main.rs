use crate::syntax::lexer::Lexer;
use crate::syntax::parser::AstParser;

mod syntax;

fn main() {
    let source = r#"def double(x)
        return x * 2
    end
    "#;

    let mut tokens = Lexer::tokenize(source).unwrap();
    let exprs = AstParser::parse(&mut tokens).unwrap();

    println!("{:?}", exprs);
}
