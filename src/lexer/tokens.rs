#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IAssociativity {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IConstants {
    Pi,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IFunctions {
    Max,
    Min,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IToken {
    Pow,
    Mul,
    Div,
    Add,
    Sub,
    Fun(IFunctions),
    LPar,
    RPar,
    Num,
    Const(IConstants),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Token {
    pub token_type: IToken,
    pub associativity: Option<IAssociativity>,
    pub precedence: Option<u8>,
    pub lexeme: Option<f64>,
}
