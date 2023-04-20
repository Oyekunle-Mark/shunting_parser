use crate::ast::nodes::{Add, AstNode, Const, Div, Fun, LPar, Mul, Num, Pow, Sub};
use crate::tokenizer::tokens::{IAssociativity, IConstants, IFunctions, IToken, Token};
use std::process;
use std::vec::IntoIter;

pub struct ShuntingYardParser {
    ast: Box<dyn AstNode>,
}

impl ShuntingYardParser {
    /// Builds the ShuntingYardParser struct and the AST from the
    /// token_stream which is stored in the private ast field.
    /// The algorithm is a variation of the popular Shunting Yard Algorithm(https://en.wikipedia.org/wiki/Shunting_yard_algorithm)
    /// for expression parsing.
    /// The modification is for generating an AST(of sort) instead
    /// of producing result in Reverse Polish notation.
    pub fn build(token_stream: &mut IntoIter<Token>) -> Self {
        let mut value_stack: Vec<Box<dyn AstNode>> = Vec::new();
        let mut operator_stack: Vec<Box<dyn AstNode>> = Vec::new();

        let handle_missing_value = || {
            eprintln!("Imbalance input supplied");
            process::exit(1);
        };

        while let Some(token) = token_stream.next() {
            match token.token_type {
                IToken::Num => value_stack.push(Box::new(Num { token })),
                IToken::Const(const_type) => match const_type {
                    IConstants::Pi => value_stack.push(Box::new(Const { token })),
                },
                IToken::Fun(function_type) => match function_type {
                    IFunctions::Min => operator_stack.push(Box::new(Fun { token })),
                    IFunctions::Max => operator_stack.push(Box::new(Fun { token })),
                },
                IToken::Add | IToken::Sub | IToken::Div | IToken::Mul | IToken::Pow => {
                    while !operator_stack.is_empty()
                        && operator_stack.last().unwrap().token_type() != IToken::LPar
                        && operator_stack.last().unwrap().precedence() >= token.precedence
                        && token.associativity.unwrap() == IAssociativity::Left
                    {
                        Self::evaluate_operator(&mut value_stack, &mut operator_stack);
                    }

                    match token.token_type {
                        IToken::Add => operator_stack.push(Box::new(Add { token })),
                        IToken::Sub => operator_stack.push(Box::new(Sub { token })),
                        IToken::Div => operator_stack.push(Box::new(Div { token })),
                        IToken::Mul => operator_stack.push(Box::new(Mul { token })),
                        IToken::Pow => operator_stack.push(Box::new(Pow { token })),
                        _ => panic!("Unidentified token {:#?}", token),
                    }
                }
                IToken::LPar => operator_stack.push(Box::new(LPar { token })),
                IToken::RPar => {
                    while !operator_stack.is_empty()
                        && operator_stack.last().unwrap().token_type() != IToken::LPar
                    {
                        Self::evaluate_operator(&mut value_stack, &mut operator_stack);
                    }

                    if !operator_stack.is_empty()
                        && operator_stack.last().unwrap().token_type() == IToken::LPar
                    {
                        operator_stack.pop();
                    } else {
                        panic!("Expression has imbalanced parenthesis");
                    }

                    if !operator_stack.is_empty()
                        && operator_stack.last().unwrap().token_type()
                            == IToken::Fun(IFunctions::Max)
                    {
                        let _fn_node = operator_stack.pop().unwrap();
                        let arg_2 = value_stack.pop().unwrap_or_else(handle_missing_value);
                        let arg_1 = value_stack.pop().unwrap_or_else(handle_missing_value);

                        value_stack.push(Box::new(Num {
                            token: Token {
                                token_type: IToken::Num,
                                associativity: None,
                                precedence: None,
                                literal: Some(if arg_1.evaluate() > arg_2.evaluate() {
                                    arg_1.evaluate()
                                } else {
                                    arg_2.evaluate()
                                }),
                            },
                        }));
                    } else if !operator_stack.is_empty()
                        && operator_stack.last().unwrap().token_type()
                            == IToken::Fun(IFunctions::Min)
                    {
                        let _fn_node = operator_stack.pop().unwrap();
                        let arg_2 = value_stack.pop().unwrap_or_else(handle_missing_value);
                        let arg_1 = value_stack.pop().unwrap_or_else(handle_missing_value);

                        value_stack.push(Box::new(Num {
                            token: Token {
                                token_type: IToken::Num,
                                associativity: None,
                                precedence: None,
                                literal: Some(if arg_1.evaluate() < arg_2.evaluate() {
                                    arg_1.evaluate()
                                } else {
                                    arg_2.evaluate()
                                }),
                            },
                        }));
                    }
                }
            }
        }

        while !operator_stack.is_empty() {
            if operator_stack.last().unwrap().token_type() == IToken::LPar {
                panic!("Expression has imbalanced parenthesis");
            }

            Self::evaluate_operator(&mut value_stack, &mut operator_stack);
        }

        Self {
            ast: value_stack.pop().unwrap(),
        }
    }

    /// A helper for processing operators as they are popped from the operator_stack
    /// Pops the operator, gets its operands from the value_stack and returns
    /// a Num node back to the value stack representing the result of the operation
    fn evaluate_operator(
        value_stack: &mut Vec<Box<dyn AstNode>>,
        operator_stack: &mut Vec<Box<dyn AstNode>>,
    ) {
        let handle_missing_value = || {
            eprintln!("Imbalance input supplied");
            process::exit(1);
        };

        let current_op = operator_stack.pop().unwrap();
        let arg_2 = value_stack.pop().unwrap_or_else(handle_missing_value);
        let arg_1 = value_stack.pop().unwrap_or_else(handle_missing_value);

        match current_op.token_type() {
            IToken::Add => value_stack.push(Box::new(Num {
                token: Token {
                    token_type: IToken::Num,
                    associativity: None,
                    precedence: None,
                    literal: Some(arg_1.evaluate() + arg_2.evaluate()),
                },
            })),
            IToken::Sub => value_stack.push(Box::new(Num {
                token: Token {
                    token_type: IToken::Num,
                    associativity: None,
                    precedence: None,
                    literal: Some(arg_1.evaluate() - arg_2.evaluate()),
                },
            })),
            IToken::Div => value_stack.push(Box::new(Num {
                token: Token {
                    token_type: IToken::Num,
                    associativity: None,
                    precedence: None,
                    literal: Some(arg_1.evaluate() / arg_2.evaluate()),
                },
            })),
            IToken::Mul => value_stack.push(Box::new(Num {
                token: Token {
                    token_type: IToken::Num,
                    associativity: None,
                    precedence: None,
                    literal: Some(arg_1.evaluate() * arg_2.evaluate()),
                },
            })),
            IToken::Pow => value_stack.push(Box::new(Num {
                token: Token {
                    token_type: IToken::Num,
                    associativity: None,
                    precedence: None,
                    literal: Some(arg_1.evaluate().powf(arg_2.evaluate())),
                },
            })),
            _ => panic!("Unidentified token {:#?}", current_op.token()),
        }
    }

    /// Evaluates the ast field and returns the result of the evaluation
    pub fn evaluate(&self) -> f64 {
        self.ast.evaluate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_creates_type() {
        let shuting_parser_type = ShuntingYardParser::build(
            &mut vec![
                Token {
                    token_type: IToken::Num,
                    associativity: None,
                    precedence: None,
                    literal: Some(2.0),
                },
                Token {
                    token_type: IToken::Add,
                    associativity: Some(IAssociativity::Left),
                    precedence: Some(2),
                    literal: None,
                },
                Token {
                    token_type: IToken::Num,
                    associativity: None,
                    precedence: None,
                    literal: Some(2.0),
                },
            ]
            .into_iter(),
        );

        assert_eq!(shuting_parser_type.evaluate(), 4.0);
    }

    #[test]
    fn parser_can_correctly_evaluate_expressions() {
        assert_eq!(ShuntingYardParser::build(&mut vec![
            Token {
                token_type: IToken::Num,
                associativity: None,
                precedence: None,
                literal: Some(2.0),
            },
            Token {
                token_type: IToken::Add,
                associativity: Some(IAssociativity::Left),
                precedence: Some(2),
                literal: None,
            },
            Token {
                token_type: IToken::Num,
                associativity: None,
                precedence: None,
                literal: Some(2.0),
            },
        ]
        .into_iter()).evaluate(), 4.0)
    }
}
