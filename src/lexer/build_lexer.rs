use std::error::Error;

use crate::lexer::tokens::{IAssociativity, IToken, Token};

fn build_lexer(expr: &str) -> Result<std::vec::IntoIter<Token>, Box<dyn Error>> {
    let mut tokens: Vec<Token> = vec![];

    for char in expr.chars() {
        match char {
            '^' => tokens.push(Token {
                token_type: IToken::Pow,
                associativity: Some(IAssociativity::Right),
                precedence: Some(4),
                lexeme: char.to_string(),
            }),
            '*' => tokens.push(Token {
                token_type: IToken::Mul,
                associativity: Some(IAssociativity::Left),
                precedence: Some(3),
                lexeme: char.to_string(),
            }),
            '/' => tokens.push(Token {
                token_type: IToken::Div,
                associativity: Some(IAssociativity::Left),
                precedence: Some(3),
                lexeme: char.to_string(),
            }),
            '+' => tokens.push(Token {
                token_type: IToken::Add,
                associativity: Some(IAssociativity::Left),
                precedence: Some(2),
                lexeme: char.to_string(),
            }),
            '-' => tokens.push(Token {
                token_type: IToken::Sub,
                associativity: Some(IAssociativity::Left),
                precedence: Some(2),
                lexeme: char.to_string(),
            }),
            '(' => tokens.push(Token {
                token_type: IToken::LPar,
                associativity: None,
                precedence: None,
                lexeme: char.to_string(),
            }),
            ')' => tokens.push(Token {
                token_type: IToken::RPar,
                associativity: None,
                precedence: None,
                lexeme: char.to_string(),
            }),
            _ => {
                eprintln!();
                return Err(format!("Unidentified character: {}", char).into());
            }
        }
    }

    Ok(tokens.into_iter())
}
