enum IAssociativity {
    Left,
    Right,
}

enum IToken {
    Pow,
    Mul,
    Div,
    Add,
    Sub,
    Fun,
    LPar,
    RPar,
    Num,
    Const,
}

struct Token {
    token_type: IToken,
    associativity: IAssociativity,
    precedence: Option<u8>,
    lexeme: String,
}
