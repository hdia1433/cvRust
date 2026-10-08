mod op;

use super::{Node, Conversion};
pub use op::Op;
use crate::Type;
use std::fmt::Display;

#[derive(Debug)]
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

    pub fn getLhs(&self) -> &Box<Node>
    {
        &self.lhs
    }

    pub fn getRhs(&self) -> &Box<Node>
    {
        &self.rhs
    }

    pub fn convertRhs(&mut self, toType: Type)
    {
        let rhs = std::mem::take(&mut self.rhs);

        self.rhs = Box::new(Node::Conversion(Conversion::newFromBox(rhs, toType)));
    }

    pub fn getLhsMut(&mut self) -> &mut Box<Node>
    {
        &mut self.lhs
    }

    pub fn getRhsMut(&mut self) -> &mut Box<Node>
    {
        &mut self.rhs
    }

    pub fn getOp(&self) -> &Op
    {
        &self.op
    }

    pub fn getType(&self) -> Type
    {
        self.lhs.getType()
    }
}

impl Display for BinaryOperation
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        writeln!(f, "Binary Operation")?;
        writeln!(f, "{{")?;
        writeln!(f, "Left Node: {}", self.lhs)?;
        writeln!(f, "Op: {}", self.op)?;
        writeln!(f, "Right Node: {}", self.rhs)?;
        writeln!(f, "}}")?;
        
        Ok(())
    }
}