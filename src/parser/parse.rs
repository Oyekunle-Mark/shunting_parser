use crate::ast::nodes::{Add, AstNode, Const, Div, Fun, LPar, Mul, Num, Pow, Sub};
use crate::lexer::tokens::{IAssociativity, IConstants, IFunctions, IToken, Token};
use std::vec::IntoIter;

pub fn parse_expression(expr: &mut IntoIter<Token>) -> Box<dyn AstNode> {
    let mut value_stack: Vec<Box<dyn AstNode>> = Vec::new();
    let mut operator_stack: Vec<Box<dyn AstNode>> = Vec::new();

    while let Some(token) = expr.next() {
        match token.token_type {
            IToken::Num => value_stack.push(Box::new(Num { token })),
            IToken::Const(const_type) => match const_type {
                IConstants::Pi => value_stack.push(Box::new(Const { token })),
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
                    evaluate_operator(&mut value_stack, &mut operator_stack);
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
                    && operator_stack.last().unwrap().precedence() >= token.precedence
                    && token.associativity.unwrap() == IAssociativity::Left
                {
                    evaluate_operator(&mut value_stack, &mut operator_stack);
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
                    // these arguments should be pass in the reverse order
                    // ignoring that because the MIN and MAX functions we have
                    // are both argument order agnostic
                    fn_node.set_arguments(vec![
                        value_stack.pop().unwrap(),
                        value_stack.pop().unwrap(),
                    ]);

                    value_stack.push(fn_node);
                }
            }
        }
    }

    value_stack.pop().unwrap()
}

fn evaluate_operator(
    value_stack: &mut Vec<Box<dyn AstNode>>,
    operator_stack: &mut Vec<Box<dyn AstNode>>,
) {
    let current_op = operator_stack.pop().unwrap();
    let arg_2 = value_stack.pop().unwrap();
    let arg_1 = value_stack.pop().unwrap();

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
