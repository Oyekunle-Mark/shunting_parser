use crate::lexer::tokens::{IToken, Token};

/// Every AST node must implement the evaluate method
pub trait AstNode {
    fn evaluate(&self) -> f64;
    fn precedence(&self) -> Option<u8>;
    fn token_type(&self) -> IToken;
    fn set_arguments(&mut self, args: Vec<Box<dyn AstNode>>) {
        panic!("Not implemented");
    }
}

pub struct Pow {
    pub left: Option<Box<dyn AstNode>>,
    pub right: Option<Box<dyn AstNode>>,
    pub token: Token,
}

pub struct Mul {
    pub left: Option<Box<dyn AstNode>>,
    pub right: Option<Box<dyn AstNode>>,
    pub token: Token,
}

pub struct Div {
    pub left: Option<Box<dyn AstNode>>,
    pub right: Option<Box<dyn AstNode>>,
    pub token: Token,
}

pub struct Add {
    pub left: Option<Box<dyn AstNode>>,
    pub right: Option<Box<dyn AstNode>>,
    pub token: Token,
}

pub struct Sub {
    pub left: Option<Box<dyn AstNode>>,
    pub right: Option<Box<dyn AstNode>>,
    pub token: Token,
}

pub struct Num {
    pub literal: f64,
    pub token: Token,
}

pub struct Const {
    pub literal: f64,
    pub token: Token,
}

pub struct Fun {
    pub arguments: Option<Vec<Box<dyn AstNode>>>,
    pub procedure: Box<dyn Fn(&Vec<Box<dyn AstNode>>) -> f64>,
    pub token: Token,
}

/// mere tombstone for handling parenthesis
pub struct LPar {
    pub token: Token,
}

/// Additionally provided for the Fun node to get the procedure to call on the arguments
/// in the evaluate method
impl Fun {
    fn procedure(&self) -> &Box<dyn Fn(&Vec<Box<dyn AstNode>>) -> f64> {
        &self.procedure
    }
}

impl AstNode for Const {
    fn evaluate(&self) -> f64 {
        self.literal
    }
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
    fn token_type(&self) -> IToken {
        self.token.token_type
    }
}

impl AstNode for Num {
    fn evaluate(&self) -> f64 {
        self.literal
    }
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
    fn token_type(&self) -> IToken {
        self.token.token_type
    }
}

impl AstNode for Sub {
    fn evaluate(&self) -> f64 {
        self.left.as_ref().unwrap().evaluate() - self.right.as_ref().unwrap().evaluate()
    }
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
    fn token_type(&self) -> IToken {
        self.token.token_type
    }
}

impl AstNode for Add {
    fn evaluate(&self) -> f64 {
        self.left.as_ref().unwrap().evaluate() + self.right.as_ref().unwrap().evaluate()
    }
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
    fn token_type(&self) -> IToken {
        self.token.token_type
    }
}

impl AstNode for Div {
    fn evaluate(&self) -> f64 {
        self.left.as_ref().unwrap().evaluate() / self.right.as_ref().unwrap().evaluate()
    }
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
    fn token_type(&self) -> IToken {
        self.token.token_type
    }
}

impl AstNode for Mul {
    fn evaluate(&self) -> f64 {
        self.left.as_ref().unwrap().evaluate() * self.right.as_ref().unwrap().evaluate()
    }
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
    fn token_type(&self) -> IToken {
        self.token.token_type
    }
}

impl AstNode for Pow {
    fn evaluate(&self) -> f64 {
        self.left.as_ref()
            .unwrap()
            .evaluate()
            .powf(self.right.as_ref().unwrap().evaluate())
    }
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
    fn token_type(&self) -> IToken {
        self.token.token_type
    }
}

impl AstNode for Fun {
    fn evaluate(&self) -> f64 {
        self.procedure()(&self.arguments.as_ref().unwrap())
    }
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
    fn token_type(&self) -> IToken {
        self.token.token_type
    }
    fn set_arguments(&mut self, args: Vec<Box<dyn AstNode>>) {
        self.arguments = Some(args);
    }
}

impl AstNode for LPar {
    fn evaluate(&self) -> f64 {
        panic!("Invalid operation on a LPar");
    }
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
    fn token_type(&self) -> IToken {
        self.token.token_type
    }
}
