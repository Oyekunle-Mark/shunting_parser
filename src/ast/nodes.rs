use crate::lexer::tokens::{IToken, Token};

/// Every AST node must implement the evaluate method
pub trait AstNode {
    fn evaluate(&self) -> f64 {
        panic!("evaluation not implemented for this type.");
    }
    fn precedence(&self) -> Option<u8>;
    fn token_type(&self) -> IToken;
    fn token(&self) -> Token;
    #[allow(unused_variables)]
    fn set_arguments(&mut self, args: Vec<Box<dyn AstNode>>) {
        panic!("set arguments not implemented for this type.");
    }
}

pub struct Pow {
    pub token: Token,
}

pub struct Mul {
    pub token: Token,
}

pub struct Div {
    pub token: Token,
}

pub struct Add {
    pub token: Token,
}

pub struct Sub {
    pub token: Token,
}

pub struct Num {
    pub token: Token,
}

pub struct Const {
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
        self.token.literal.unwrap()
    }
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
    fn token_type(&self) -> IToken {
        self.token.token_type
    }
    fn token(&self) -> Token {
        self.token
    }
}

impl AstNode for Num {
    fn evaluate(&self) -> f64 {
        self.token.literal.unwrap()
    }
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
    fn token_type(&self) -> IToken {
        self.token.token_type
    }
    fn token(&self) -> Token {
        self.token
    }
}

impl AstNode for Sub {
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
    fn token_type(&self) -> IToken {
        self.token.token_type
    }
    fn token(&self) -> Token {
        self.token
    }
}

impl AstNode for Add {
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
    fn token_type(&self) -> IToken {
        self.token.token_type
    }
    fn token(&self) -> Token {
        self.token
    }
}

impl AstNode for Div {
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
    fn token_type(&self) -> IToken {
        self.token.token_type
    }
    fn token(&self) -> Token {
        self.token
    }
}

impl AstNode for Mul {
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
    fn token_type(&self) -> IToken {
        self.token.token_type
    }
    fn token(&self) -> Token {
        self.token
    }
}

impl AstNode for Pow {
    fn precedence(&self) -> Option<u8> {
        self.token.precedence
    }
    fn token_type(&self) -> IToken {
        self.token.token_type
    }
    fn token(&self) -> Token {
        self.token
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
    fn token(&self) -> Token {
        self.token
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
    fn token(&self) -> Token {
        self.token
    }
}
