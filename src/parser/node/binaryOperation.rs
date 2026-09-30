use super::Node;
use crate::lexer::tokenType::TokenType;
use std::fmt::Display;

pub enum Op
{
    Assign,
}

impl Display for Op
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "=")
    }
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

    pub fn getLhs(&self) -> &Box<Node>
    {
        &self.lhs
    }

    pub fn getRhs(&self) -> &Box<Node>
    {
        &self.rhs
    }

    pub fn getType(&self) -> TokenType
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