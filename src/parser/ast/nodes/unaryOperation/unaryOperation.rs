use crate::parser::ast::{nodes::{Node, Type}, ToFile};
use super::Op;
use bitvec::prelude::BitVec;

use std::{io::Write, fmt::Display};

#[derive(Debug)]
pub struct UnaryOperation
{
    prefix: bool,
    op: Op,
    node: Box<Node>
}

impl UnaryOperation
{
    pub fn newPrefix(op: Op, node: Node) -> Self
    {
        Self {prefix: true, op, node: Box::new(node)}
    }

    pub fn getType(&self) -> Type
    {
        match self.op
        {
            Op::AddressOf => Type::Pointer(Box::new(self.node.getType())),
            Op::Dereference => 
            {
                let Type::Pointer(ptr) = self.node.getType() else
                {
                    panic!("Cannot get dereferenced type of a non pointer")
                };

                *ptr
            }
        }
    }

    pub fn getOp(&self) -> &Op
    {
        &self.op
    }

    pub fn getNode(&self) -> &Node
    {
        &self.node
    }

    pub fn getNodeMut(&mut self) -> &mut Node
    {
        &mut self.node
    }
}

impl Display for UnaryOperation
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        if self.prefix
        {
            write!(f, "Unary Operation: {}{}", self.op, self.node)
        }
        else 
        {
            write!(f, "Unary Operation: {}{}", self.node, self.op)
        }
    }
}

impl ToFile for UnaryOperation
{
    fn toFile(&self, f: &mut std::fs::File, indents: &mut BitVec, last: bool) -> std::io::Result<()> 
    {
        let indent:String = Self::getIndent(indents, last);

        write!(f, "{}Unary Operation", indent)?;

        let mut childIndents:BitVec = indents.clone();
        childIndents.push(!last);

        if self.prefix
        {
            write!(f, "{}{}", Self::getIndent(&childIndents, false), self.op)?;
        }

        self.node.toFile(f, &mut childIndents, self.prefix)?;

        if !self.prefix
        {
            write!(f, "{}{}", Self::getIndent(&childIndents, true), self.op)?;
        }

        Ok(())
    }
}