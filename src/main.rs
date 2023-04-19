use shunting_parser::evaluate_expression;
use std::{env, error::Error, process};

pub mod ast;
pub mod lexer;
pub mod parser;

const PARSER_OPTION: &str = "eval";

fn main() {
    let args: Vec<String> = env::args().collect();

    let expr = parse_config(&args).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });

    println!("> {}", evaluate_expression(expr));
}

fn parse_config(args: &Vec<String>) -> Result<&str, Box<dyn Error>> {
    if args.len() != 3 || args[1] != PARSER_OPTION {
        eprintln!("Usage: <cargo run>|<./executable> eval \"<expression to be evaluated>\"");
        return Err("Error: Invalid option provided.".into());
    }

    Ok(&args[2])
}
