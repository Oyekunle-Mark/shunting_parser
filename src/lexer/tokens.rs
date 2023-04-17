#[derive(Debug, PartialEq)]
pub enum IAssociativity {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
pub enum IConstants {
    Pi,
}

#[derive(Debug, PartialEq)]
pub enum IFunctions {
    Max,
    Min,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: IToken,
    pub associativity: Option<IAssociativity>,
    pub precedence: Option<u8>,
    pub lexeme: String,
}
