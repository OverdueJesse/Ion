use std::fmt::{self, Display, Formatter};

pub struct Program {
    pub tree: Vec<Node>,
}

impl Program {
    pub fn new() -> Program {
        Program { tree: vec![] }
    }
    pub fn push_node(&mut self, node: Node) {
        self.tree.push(node);
    }
    pub fn print_nodes(&self) {
        println!("{self}");
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut s: String = String::new();
        for node in self.tree.iter() {
            s.push_str(node.to_string().as_str());
            s.push_str("\n");
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Literal(Literal),
    Declaration(Declaration),
    BinaryExpr {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
}

impl Display for Node {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Sub,
    Div,
    Mult,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Float(f32),
    BigFloat(f64),
    String(String),
    Char(char),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub symbol: String,
    pub value: Box<Node>,
}
