use crate::parser::ast::{nodes::{Node, Conversion}, Type};
pub use super::Op;
use std::{fmt::Display, io::Write};
use crate::parser::ast::ToFile;
use bitvec::prelude::BitVec;

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

    pub fn convertLhs(&mut self, toType: Type)
    {
        let lhs = std::mem::take(&mut self.lhs);

        self.lhs = Box::new(Node::Conversion(Conversion::newFromBox(lhs, toType)));
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
        match self.op
        {
            Op::Equals => Type::Bool,
            _ => self.lhs.getType()
        }
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

impl ToFile for BinaryOperation
{
    fn toFile(&self, f: &mut std::fs::File, indents: &mut BitVec, last: bool) -> std::io::Result<()>
    {
        let indent:String = Self::getIndent(indents, last);

        write!(f, "{}Binary Operation", indent)?;

        let mut childIndents:BitVec = indents.clone();

        childIndents.push(!last);

        self.lhs.toFile(f, &mut childIndents, false)?;
        write!(f, "{}{}", Self::getIndent(&childIndents, false), self.op)?;
        self.rhs.toFile(f, &mut childIndents, true)?;

        Ok(())
    }
}