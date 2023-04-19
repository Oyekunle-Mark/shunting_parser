use crate::lexer::tokens::Token;

/// Every AST node must implement the evaluate method
pub trait AstNode {
    fn evaluate(&self) -> f64;
    fn precedence(&self) -> Option<u8>;
}

pub struct Pow {
    left: Box<dyn AstNode>,
    right: Box<dyn AstNode>,
    token: Token,
}

pub struct Mul {
    left: Box<dyn AstNode>,
    right: Box<dyn AstNode>,
    token: Token,
}

pub struct Div {
    left: Box<dyn AstNode>,
    right: Box<dyn AstNode>,
    token: Token,
}

pub struct Add {
    left: Box<dyn AstNode>,
    right: Box<dyn AstNode>,
    token: Token,
}

pub struct Sub {
    left: Box<dyn AstNode>,
    right: Box<dyn AstNode>,
    token: Token,
}

pub struct Num {
    literal: f64,
    token: Token,
}

pub struct Const {
    literal: f64,
    token: Token,
}

pub struct Fun {
    arguments: Option<Vec<Box<dyn AstNode>>>,
    procedure: Box<dyn Fn(&Vec<Box<dyn AstNode>>) -> f64>,
    token: Token,
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
}

impl AstNode for Num {
    fn evaluate(&self) -> f64 {
        self.literal
    }
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
}

impl AstNode for Sub {
    fn evaluate(&self) -> f64 {
        self.left.evaluate() - self.right.evaluate()
    }
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
}

impl AstNode for Add {
    fn evaluate(&self) -> f64 {
        self.left.evaluate() + self.right.evaluate()
    }
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
}

impl AstNode for Div {
    fn evaluate(&self) -> f64 {
        self.left.evaluate() / self.right.evaluate()
    }
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
}

impl AstNode for Mul {
    fn evaluate(&self) -> f64 {
        self.left.evaluate() * self.right.evaluate()
    }
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
}

impl AstNode for Pow {
    fn evaluate(&self) -> f64 {
        self.left.evaluate().powf(self.right.evaluate())
    }
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
}

impl AstNode for Fun {
    fn evaluate(&self) -> f64 {
        self.procedure()(&self.arguments.unwrap())
    }
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
}
