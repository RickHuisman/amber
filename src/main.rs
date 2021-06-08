use crate::syntax::lexer::Lexer;

mod syntax;

fn main() {
    // let source = r#"let x = 10"#;
    let source = r#"2
    10
    3.33"#;
    let tokens = Lexer::tokenize(source);
    println!("{:?}", tokens);
}
