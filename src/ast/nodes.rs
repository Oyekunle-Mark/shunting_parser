use crate::lexer::tokens::{IToken, Token};

pub trait AstNode {
    fn evaluate(&self) -> f64 {
        panic!("evaluation not implemented for this type.");
    }
    fn precedence(&self) -> Option<u8>;
    fn token_type(&self) -> IToken;
    fn token(&self) -> Token;
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
    pub token: Token,
}

/// mere tombstone for handling parenthesis
pub struct LPar {
    pub token: Token,
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

impl AstNode for LPar {
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
