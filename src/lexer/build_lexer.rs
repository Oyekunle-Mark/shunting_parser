use std::error::Error;

use crate::lexer::tokens::{IAssociativity, IConstants, IFunctions, IToken, Token};

pub fn build_lexer(expr: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    let mut tokens: Vec<Token> = vec![];
    let mut number = String::new();
    let mut identifier = String::new();

    for char in expr.chars() {
        match char {
            '^' => {
                clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);

                tokens.push(Token {
                    token_type: IToken::Pow,
                    associativity: Some(IAssociativity::Right),
                    precedence: Some(4),
                    lexeme: char.to_string(),
                });
            }
            '*' => {
                clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);

                tokens.push(Token {
                    token_type: IToken::Mul,
                    associativity: Some(IAssociativity::Left),
                    precedence: Some(3),
                    lexeme: char.to_string(),
                });
            }
            '/' => {
                clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);

                tokens.push(Token {
                    token_type: IToken::Div,
                    associativity: Some(IAssociativity::Left),
                    precedence: Some(3),
                    lexeme: char.to_string(),
                });
            }
            '+' => {
                clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);

                tokens.push(Token {
                    token_type: IToken::Add,
                    associativity: Some(IAssociativity::Left),
                    precedence: Some(2),
                    lexeme: char.to_string(),
                })
            }
            '-' => {
                clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);

                tokens.push(Token {
                    token_type: IToken::Sub,
                    associativity: Some(IAssociativity::Left),
                    precedence: Some(2),
                    lexeme: char.to_string(),
                });
            }
            '(' => {
                clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);

                tokens.push(Token {
                    token_type: IToken::LPar,
                    associativity: None,
                    precedence: None,
                    lexeme: char.to_string(),
                });
            }
            ')' => {
                clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);

                tokens.push(Token {
                    token_type: IToken::RPar,
                    associativity: None,
                    precedence: None,
                    lexeme: char.to_string(),
                });
            }
            char if char.is_numeric() => {
                match clear_identifier(&mut identifier) {
                    Some(token) => tokens.push(token),
                    None => (),
                }

                number.push(char);
            }
            char if char.is_alphabetic() => {
                match clear_number(&mut number) {
                    Some(token) => tokens.push(token),
                    None => (),
                }

                identifier.push(char);
            }
            _ => {
                return Err(format!("Unidentified character: {}", char).into());
            }
        }
    }

    clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);

    Ok(tokens.into_iter().collect())
}

fn clear_identifier(identifier: &mut String) -> Option<Token> {
    if !identifier.is_empty() {
        return None;
    }

    let token = match identifier.as_str() {
        "min" => Token {
            token_type: IToken::Fun(IFunctions::Min),
            associativity: None,
            precedence: None,
            lexeme: identifier.clone(),
        },
        "max" => Token {
            token_type: IToken::Fun(IFunctions::Max),
            associativity: None,
            precedence: None,
            lexeme: identifier.clone(),
        },
        "pi" => Token {
            token_type: IToken::Const(IConstants::Pi),
            associativity: None,
            precedence: None,
            lexeme: identifier.clone(),
        },
        _ => panic!("Unidentified identifier: {}", identifier),
    };

    identifier.clear();

    Some(token)
}

fn clear_number(number: &mut String) -> Option<Token> {
    if !number.is_empty() {
        return None;
    }

    let token = Token {
        token_type: IToken::Num,
        associativity: None,
        precedence: None,
        lexeme: number.clone(),
    };

    number.clear();

    Some(token)
}

fn clear_identifier_or_number(
    identifier: &mut String,
    number: &mut String,
    tokens: &mut Vec<Token>,
) {
    match clear_identifier(identifier) {
        Some(token) => tokens.push(token),
        None => (),
    }

    match clear_number(number) {
        Some(token) => tokens.push(token),
        None => (),
    }
}
