// use std::error::Error;
use std::io::{Error, ErrorKind};

use crate::tokenizer::tokens::{IAssociativity, IConstants, IFunctions, IToken, Token};

pub struct Tokenizer {
    token_stream: Result<std::vec::IntoIter<Token>, Error>,
}

impl Tokenizer {
    pub fn build(expr: &str) -> Self {
        let mut tokens: Vec<Token> = vec![];
        let mut number = String::new();
        let mut identifier = String::new();

        for char in expr.chars() {
            match char {
                '^' => {
                    Self::clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);

                    tokens.push(Token {
                        token_type: IToken::Pow,
                        associativity: Some(IAssociativity::Right),
                        precedence: Some(4),
                        literal: None,
                    });
                }
                '*' => {
                    Self::clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);

                    tokens.push(Token {
                        token_type: IToken::Mul,
                        associativity: Some(IAssociativity::Left),
                        precedence: Some(3),
                        literal: None,
                    });
                }
                '/' => {
                    Self::clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);

                    tokens.push(Token {
                        token_type: IToken::Div,
                        associativity: Some(IAssociativity::Left),
                        precedence: Some(3),
                        literal: None,
                    });
                }
                '+' => {
                    Self::clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);

                    tokens.push(Token {
                        token_type: IToken::Add,
                        associativity: Some(IAssociativity::Left),
                        precedence: Some(2),
                        literal: None,
                    })
                }
                '-' => {
                    Self::clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);

                    tokens.push(Token {
                        token_type: IToken::Sub,
                        associativity: Some(IAssociativity::Left),
                        precedence: Some(2),
                        literal: None,
                    });
                }
                '(' => {
                    Self::clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);

                    tokens.push(Token {
                        token_type: IToken::LPar,
                        associativity: None,
                        precedence: None,
                        literal: None,
                    });
                }
                ')' => {
                    Self::clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);

                    tokens.push(Token {
                        token_type: IToken::RPar,
                        associativity: None,
                        precedence: None,
                        literal: None,
                    });
                }
                ',' | ' ' => {
                    Self::clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);
                    continue;
                }
                char if char.is_numeric() || char == '.' => {
                    match Self::clear_identifier(&mut identifier) {
                        Some(token) => tokens.push(token),
                        None => (),
                    }

                    number.push(char);
                }
                char if char.is_alphabetic() => {
                    match Self::clear_number(&mut number) {
                        Some(token) => tokens.push(token),
                        None => (),
                    }

                    identifier.push(char);
                }
                _ => {
                    return Self {
                        token_stream: Err(Error::new::<_>(
                            ErrorKind::InvalidData,
                            format!("Unidentified character: {}", char),
                        )),
                    };
                }
            }
        }

        Self::clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);

        Self {
            token_stream: Ok(tokens.into_iter()),
        }
    }

    /// Creates a token using the current string in the identifier
    /// variable and clears the string.
    /// Token can be a function or constant depending on the lexeme.
    fn clear_identifier(identifier: &mut String) -> Option<Token> {
        if identifier.is_empty() {
            return None;
        }

        let token = match identifier.as_str() {
            "min" => Token {
                token_type: IToken::Fun(IFunctions::Min),
                associativity: None,
                precedence: None,
                literal: None,
            },
            "max" => Token {
                token_type: IToken::Fun(IFunctions::Max),
                associativity: None,
                precedence: None,
                literal: None,
            },
            "pi" => Token {
                token_type: IToken::Const(IConstants::Pi),
                associativity: None,
                precedence: None,
                literal: Some(3.14159265359),
            },
            _ => panic!("Unidentified identifier: {}", identifier),
        };

        identifier.clear();

        Some(token)
    }

    /// Creates a number token using the current string in the number
    /// variable and clears the string.
    fn clear_number(number: &mut String) -> Option<Token> {
        if number.is_empty() {
            return None;
        }

        let token = Token {
            token_type: IToken::Num,
            associativity: None,
            precedence: None,
            literal: Some(number.parse::<f64>().unwrap()),
        };

        number.clear();

        Some(token)
    }

    /// A convenience over calling clear_identifier and clear_number separately
    fn clear_identifier_or_number<'a>(
        identifier: &'a mut String,
        number: &'a mut String,
        tokens: &mut Vec<Token>,
    ) {
        match Self::clear_identifier(identifier) {
            Some(token) => tokens.push(token),
            None => (),
        }

        match Self::clear_number(number) {
            Some(token) => tokens.push(token),
            None => (),
        }
    }

    pub fn tokens(&mut self) -> &mut Result<std::vec::IntoIter<Token>, Error> {
        &mut self.token_stream
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clear_number_returns_none_when_number_empty() {
        let mut number = String::new();

        assert!(Tokenizer::clear_number(&mut number).is_none());

        let mut number = String::from("");

        assert!(Tokenizer::clear_number(&mut number).is_none());
    }

    #[test]
    fn clear_number_returns_token_for_non_empty_lexeme_and_clears_string() {
        let mut number = String::from("3.101");
        let token = Tokenizer::clear_number(&mut number).unwrap();

        assert_eq!(
            Token {
                token_type: IToken::Num,
                associativity: None,
                precedence: None,
                literal: Some(3.101),
            },
            token
        );

        assert!(number.is_empty());
    }

    #[test]
    fn clear_identifier_returns_none_when_identifier_is_empty() {
        let mut identifier = String::new();

        assert!(Tokenizer::clear_identifier(&mut identifier).is_none());

        let mut identifier = String::from("");

        assert!(Tokenizer::clear_identifier(&mut identifier).is_none());
    }

    #[test]
    fn clear_identifier_returns_token_and_clears_string() {
        // min token
        let mut identifier = String::from("min");
        let token = Tokenizer::clear_identifier(&mut identifier).unwrap();

        assert_eq!(
            Token {
                token_type: IToken::Fun(IFunctions::Min),
                associativity: None,
                precedence: None,
                literal: None,
            },
            token
        );

        assert!(identifier.is_empty());

        // max token
        let mut identifier = String::from("max");
        let token = Tokenizer::clear_identifier(&mut identifier).unwrap();

        assert_eq!(
            Token {
                token_type: IToken::Fun(IFunctions::Max),
                associativity: None,
                precedence: None,
                literal: None,
            },
            token
        );

        assert!(identifier.is_empty());

        // pi token
        let mut identifier = String::from("pi");
        let token = Tokenizer::clear_identifier(&mut identifier).unwrap();

        assert_eq!(
            Token {
                token_type: IToken::Const(IConstants::Pi),
                associativity: None,
                precedence: None,
                literal: Some(3.14159265359),
            },
            token
        );

        assert!(identifier.is_empty());
    }

    #[test]
    #[should_panic(expected = "Unidentified identifier:")]
    fn clear_identifier_panics_at_invalid_identifier() {
        let mut identifier = String::from("invalid");
        let _token = Tokenizer::clear_identifier(&mut identifier).unwrap();
    }

    #[test]
    fn clear_identifier_or_number_can_create_token_for_number() {
        let mut number = String::from("0.331");
        let mut identifier = String::new();
        let mut tokens = vec![];

        Tokenizer::clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);

        assert_eq!(
            vec![Token {
                token_type: IToken::Num,
                associativity: None,
                precedence: None,
                literal: Some(0.331),
            }],
            tokens
        );

        assert!(number.is_empty());
    }

    #[test]
    fn clear_identifier_or_number_can_create_token_for_identifier() {
        let mut number = String::new();
        let mut identifier = String::from("pi");
        let mut tokens = vec![];

        Tokenizer::clear_identifier_or_number(&mut identifier, &mut number, &mut tokens);

        assert_eq!(
            vec![Token {
                token_type: IToken::Const(IConstants::Pi),
                associativity: None,
                precedence: None,
                literal: Some(3.14159265359),
            }],
            tokens
        );

        assert!(number.is_empty());
    }

    #[test]
    fn build_lexer_returns_tokens() {
        let mut binding = Tokenizer::build("2+2(3-1)^min(1,0.1)");
        let token_stream = binding.tokens().as_mut().unwrap();

        assert_eq!(
            vec![
                Token {
                    token_type: IToken::Num,
                    associativity: None,
                    precedence: None,
                    literal: Some(2.0),
                },
                Token {
                    token_type: IToken::Add,
                    associativity: Some(IAssociativity::Left,),
                    precedence: Some(2,),
                    literal: None,
                },
                Token {
                    token_type: IToken::Num,
                    associativity: None,
                    precedence: None,
                    literal: Some(2.0),
                },
                Token {
                    token_type: IToken::LPar,
                    associativity: None,
                    precedence: None,
                    literal: None,
                },
                Token {
                    token_type: IToken::Num,
                    associativity: None,
                    precedence: None,
                    literal: Some(3.0),
                },
                Token {
                    token_type: IToken::Sub,
                    associativity: Some(IAssociativity::Left,),
                    precedence: Some(2,),
                    literal: None,
                },
                Token {
                    token_type: IToken::Num,
                    associativity: None,
                    precedence: None,
                    literal: Some(1.0),
                },
                Token {
                    token_type: IToken::RPar,
                    associativity: None,
                    precedence: None,
                    literal: None,
                },
                Token {
                    token_type: IToken::Pow,
                    associativity: Some(IAssociativity::Right,),
                    precedence: Some(4,),
                    literal: None,
                },
                Token {
                    token_type: IToken::Fun(IFunctions::Min,),
                    associativity: None,
                    precedence: None,
                    literal: None,
                },
                Token {
                    token_type: IToken::LPar,
                    associativity: None,
                    precedence: None,
                    literal: None,
                },
                Token {
                    token_type: IToken::Num,
                    associativity: None,
                    precedence: None,
                    literal: Some(1.0),
                },
                Token {
                    token_type: IToken::Num,
                    associativity: None,
                    precedence: None,
                    literal: Some(0.1),
                },
                Token {
                    token_type: IToken::RPar,
                    associativity: None,
                    precedence: None,
                    literal: None,
                },
            ],
            token_stream.collect::<Vec<Token>>()
        );
    }

    #[test]
    fn build_lexer_returns_error_result_at_unrecognized_token() {
        let mut binding = Tokenizer::build("2+2(3-1)_^min(1,0.1)");
        let token_stream = binding.tokens();

        assert!(token_stream.is_err());
    }
}
