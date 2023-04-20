use crate::shunting_yard_parser::parse::ShuntingYardParser;
use crate::tokenizer::tokenize::Tokenizer;
use std::process;

pub mod ast;
pub mod shunting_yard_parser;
pub mod tokenizer;

/// Evaluates the string slice representing the expression using the
/// ShuntingYardParser type from the shunting_yard_parser module
pub fn evaluate_expression_shunting_yard(expr: &str) -> f64 {
    let mut binding = Tokenizer::build(expr);
    let tokens = binding.tokens().as_mut().unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });

    ShuntingYardParser::build(tokens).evaluate()
}
