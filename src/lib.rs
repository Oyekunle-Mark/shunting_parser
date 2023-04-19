use crate::lexer::build_lexer::build_lexer;
use crate::parser::parse::parse_expression;
use std::process;

pub mod ast;
pub mod lexer;
pub mod parser;

pub fn evaluate_expression(expr: &str) -> f64 {
    let mut tokens = build_lexer(expr).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });

    parse_expression(&mut tokens).evaluate()
}
