use std::{env, error::Error, process};

pub mod lexer;

const PARSER_OPTION: &str = "eval";

fn main() {
    let args: Vec<String> = env::args().collect();
    let expr = parse_config(&args).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });

    print!("Expression to be evaluated is: {}", expr);
}

fn parse_config(args: &Vec<String>) -> Result<&str, Box<dyn Error>> {
    if args.len() != 3 || args[1] != PARSER_OPTION {
        eprintln!("Usage: <cargo run>|<./executable> eval \"<expression to be evaluated>\"");
        return Err("Error: Invalid option provided.".into());
    }

    Ok(&args[2])
}
