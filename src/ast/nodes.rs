/// Every AST node must implement the evaluate method
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
    procedure: Box<dyn Fn(&Vec<Box<dyn AstNode>>) -> f64>,
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
}

impl AstNode for Num {
    fn evaluate(&self) -> f64 {
        self.literal
    }
}

impl AstNode for Sub {
    fn evaluate(&self) -> f64 {
        self.left.evaluate() - self.right.evaluate()
    }
}

impl AstNode for Add {
    fn evaluate(&self) -> f64 {
        self.left.evaluate() + self.right.evaluate()
    }
}

impl AstNode for Div {
    fn evaluate(&self) -> f64 {
        self.left.evaluate() / self.right.evaluate()
    }
}

impl AstNode for Mul {
    fn evaluate(&self) -> f64 {
        self.left.evaluate() * self.right.evaluate()
    }
}

impl AstNode for Pow {
    fn evaluate(&self) -> f64 {
        self.left.evaluate().powf(self.right.evaluate())
    }
}

impl AstNode for Fun {
    fn evaluate(&self) -> f64 {
        self.procedure()(&self.arguments)
    }
}
