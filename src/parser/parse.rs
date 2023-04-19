use crate::ast::nodes::{Add, AstNode, Const, Div, Fun, LPar, Mul, Num, Pow, Sub};
use crate::lexer::tokens::{IAssociativity, IConstants, IFunctions, IToken, Token};
use std::vec::IntoIter;

pub fn parse_expression(expr: &mut IntoIter<Token>) -> Box<dyn AstNode> {
    let mut value_stack: Vec<Box<dyn AstNode>> = Vec::new();
    let mut operator_stack: Vec<Box<dyn AstNode>> = Vec::new();

    while let Some(token) = expr.next() {
        match token.token_type {
            IToken::Num => value_stack.push(Box::new(Num {
                literal: token.literal.unwrap(),
                token,
            })),
            IToken::Const(const_type) => match const_type {
                IConstants::Pi => value_stack.push(Box::new(Const {
                    literal: token.literal.unwrap(),
                    token,
                })),
            },
            IToken::Fun(function_type) => match function_type {
                IFunctions::Min => operator_stack.push(Box::new(Fun {
                    token,
                    arguments: None,
                    procedure: Box::new(|args| {
                        let first = args[0].evaluate();
                        let second = args[1].evaluate();
                        if first < second {
                            first
                        } else {
                            second
                        }
                    }),
                })),
                IFunctions::Max => operator_stack.push(Box::new(Fun {
                    token,
                    arguments: None,
                    procedure: Box::new(|args| {
                        let first = args[0].evaluate();
                        let second = args[1].evaluate();
                        if first > second {
                            first
                        } else {
                            second
                        }
                    }),
                })),
            },
            IToken::Add | IToken::Sub | IToken::Div | IToken::Mul | IToken::Pow => {
                while !operator_stack.is_empty()
                    && operator_stack.last().unwrap().token_type() != IToken::LPar
                    && operator_stack.last().unwrap().precedence() >= token.precedence
                    && token.associativity.unwrap() == IAssociativity::Left
                {
                    // return to build nodes from the popped operators
                    value_stack.push(operator_stack.pop().unwrap());
                }

                match token.token_type {
                    IToken::Add => operator_stack.push(Box::new(Add {
                        token,
                        left: None,
                        right: None,
                    })),
                    IToken::Sub => operator_stack.push(Box::new(Sub {
                        token,
                        left: None,
                        right: None,
                    })),
                    IToken::Div => operator_stack.push(Box::new(Div {
                        token,
                        left: None,
                        right: None,
                    })),
                    IToken::Mul => operator_stack.push(Box::new(Mul {
                        token,
                        left: None,
                        right: None,
                    })),
                    IToken::Pow => operator_stack.push(Box::new(Pow {
                        token,
                        left: None,
                        right: None,
                    })),
                    _ => panic!("Unidentified token {:#?}", token),
                }
            }
            IToken::LPar => operator_stack.push(Box::new(LPar { token })),
            IToken::RPar => {
                while !operator_stack.is_empty()
                    && operator_stack.last().unwrap().token_type() != IToken::LPar
                    && operator_stack.last().unwrap().precedence() >= token.precedence
                    && token.associativity.unwrap() == IAssociativity::Left
                {
                    // return to build nodes from the popped operators
                    value_stack.push(operator_stack.pop().unwrap());
                }

                if !operator_stack.is_empty()
                    && operator_stack.last().unwrap().token_type() == IToken::LPar
                {
                    operator_stack.pop();
                } else {
                    panic!("Expression has imbalanced parenthesis");
                }

                if !operator_stack.is_empty()
                    && operator_stack.last().unwrap().token_type() == IToken::Fun(IFunctions::Max)
                    || operator_stack.last().unwrap().token_type() == IToken::Fun(IFunctions::Min)
                {
                    let mut fn_node = operator_stack.pop().unwrap();
                    fn_node.set_arguments(vec![
                        value_stack.pop().unwrap(),
                        value_stack.pop().unwrap(),
                    ]);
                }
            }
        }
    }

    value_stack.pop().unwrap()
}
