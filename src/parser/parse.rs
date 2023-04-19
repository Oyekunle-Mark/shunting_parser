use crate::ast::nodes::{AstNode, Num};
use crate::lexer::tokens::Token;
use std::vec::IntoIter;

pub fn parse_expression(expr: &IntoIter<Token>) -> Box<dyn AstNode> {
    // placeholder
    return Box::new(Num { literal: 2.0 });
}
