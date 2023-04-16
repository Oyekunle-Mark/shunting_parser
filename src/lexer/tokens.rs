#[derive(Debug)]
pub enum IAssociativity {
    Left,
    Right,
}

#[derive(Debug)]
pub enum IConstants {
    Pi,
}

#[derive(Debug)]
pub enum IFunctions {
    Max,
    Min,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Token {
    pub token_type: IToken,
    pub associativity: Option<IAssociativity>,
    pub precedence: Option<u8>,
    pub lexeme: String,
}
