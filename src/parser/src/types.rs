pub struct Program {
    tree: Vec<Node>,
}

impl Program {
    pub fn new() -> Program {
        Program { tree: vec![] }
    }
    pub fn push_node(&mut self, node: Node) {
        self.tree.push(node);
    }
}

pub enum Node {
    Literal(Literal),
    Assignment(Assignment),
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

pub enum Operator {
    Add,
    Sub,
    Div,
    Mult,
}

pub enum Literal {
    Float(f32),
    BigFloat(f64),
    String(String),
    Char(char),
    Boolean(bool),
}

pub struct Assignment {
    symbol: String,
    value: Box<Node>,
}
