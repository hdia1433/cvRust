use super::Node;

pub enum Op
{
    Assign,
}

pub struct BinaryOperation
{
    lhs: Box<Node>,
    op: Op,
    rhs: Box<Node>
}

impl BinaryOperation
{
    pub fn new(lhs: Node, op: Op, rhs: Node) -> Self
    {
        Self {lhs: Box::new(lhs), op, rhs: Box::new(rhs)}
    }
}