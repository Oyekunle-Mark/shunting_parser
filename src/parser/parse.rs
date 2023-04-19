use crate::ast::nodes::{AstNode, Const, Num};
use crate::lexer::tokens::{IConstants, IToken, Token};
use std::vec::IntoIter;

pub fn parse_expression(expr: &IntoIter<Token>) -> Box<dyn AstNode> {
    let mut value_stack: Vec<Box<dyn AstNode>> = Vec::new();
    let mut operator_stack = Vec::new();

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
        }
    }

    return Box::new(Num { literal: 2.1 });
}
