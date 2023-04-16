enum IAssociativity {
    Left,
    Right,
}

enum IConstants {
    Pi,
}

enum IFunctions {
    Max,
    Min,
}

enum IToken {
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

struct Token {
    token_type: IToken,
    associativity: IAssociativity,
    precedence: Option<u8>,
    lexeme: String,
}
