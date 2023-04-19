use crate::ast::nodes::{AstNode, Const, Fun, Num};
use crate::lexer::tokens::{IConstants, IFunctions, IToken, Token};
use std::vec::IntoIter;

pub fn parse_expression(expr: &IntoIter<Token>) -> Box<dyn AstNode> {
    let mut value_stack: Vec<Box<dyn AstNode>> = Vec::new();
    let mut operator_stack: Vec<Box<dyn AstNode>> = Vec::new();

    while let Some(token) = expr.next() {
        match token.token_type {
            IToken::Num => value_stack.push(Box::new(Num {
                token,
                literal: token.lexeme.parse::<f64>().unwrap(),
            })),
            IToken::Const(const_type) => match const_type {
                IConstants::Pi => value_stack.push(Box::new(Const {
                    token,
                    literal: token.lexeme.parse::<f64>().unwrap(),
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
        }
    }

    value_stack.first().unwrap()
}
