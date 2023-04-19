trait AstNode {
    fn evaluate(&self) -> f64;
}

struct Pow {
    left: Box<dyn AstNode>,
    right: Box<dyn AstNode>,
}

struct Mul {
    left: Box<dyn AstNode>,
    right: Box<dyn AstNode>,
}

struct Div {
    left: Box<dyn AstNode>,
    right: Box<dyn AstNode>,
}

struct Add {
    left: Box<dyn AstNode>,
    right: Box<dyn AstNode>,
}

struct Sub {
    left: Box<dyn AstNode>,
    right: Box<dyn AstNode>,
}

struct Num {
    literal: f64,
}

struct Const {
    literal: f64,
}

struct Fun {
    arguments: Vec<Box<dyn AstNode>>,
}
