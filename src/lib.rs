use crate::lexer::build_lexer::build_lexer;
use crate::shunting_yard_parser::parse::ShuntingYardParser;
use std::process;

pub mod ast;
pub mod lexer;
pub mod shunting_yard_parser;

pub fn evaluate_expression(expr: &str) -> f64 {
    let mut tokens = build_lexer(expr).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });

    ShuntingYardParser::build(&mut tokens).evaluate()
}
