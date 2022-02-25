use parser::Parse;
use parser::ToNum;

mod lexer;
mod parser;

pub fn parse(input: &str) -> Result<usize, String> {
    let tokens = lexer::lex(input)?;
    let tree = parser::Num::parse(&tokens);

    match tree {
        Some(t) => Ok(t.0.to_num()),
        None => Err("No Valid Number Found".into())
    }
}
