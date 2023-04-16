pub enum IAssociativity {
    Left,
    Right,
}

pub enum IConstants {
    Pi,
}

pub enum IFunctions {
    Max,
    Min,
}

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

pub struct Token {
    pub token_type: IToken,
    pub associativity: IAssociativity,
    pub precedence: Option<u8>,
    pub lexeme: String,
}
