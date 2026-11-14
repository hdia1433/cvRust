use crate::{errors::{CvError::{self, CompilerError, RuntimeError}, CompilerError::MutTypeError, RuntimeError::{InvalidDeref, InvalidSubscript}}, parser::ast::{ToFile, nodes::{Node, Type}}};
use super::Op;
use bitvec::prelude::BitVec;

use std::{borrow::Cow, fmt::Display, io::Write, ops::Deref};

#[derive(Debug, Clone)]
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

    pub fn newPostfix(op: Op, node: Node) -> Self
    {
        Self {prefix: false, op, node: Box::new(node)}
    }

    pub fn getType(&self) -> Cow<'_, Type>
    {
        match self.op
        {
            Op::AddressOf => Cow::Owned(Type::Pointer(Box::new(self.node.getType().deref().clone()))),
            Op::Dereference => 
            {
                if !matches!(&*self.node.getType(), Type::Pointer(_))
                {
                    panic!("Cannot get dereferenced type of a non pointer")
                };

                self.node.getType()
            },
            Op::Subscript(_) =>
            {
                if !matches!(&*self.node.getType(), Type::Array(_, _))
                {
                    panic!("Cannot get subscript type of a non array")
                };

                self.node.getType()
            }
        }
    }

    pub fn getTypeMut(&mut self) -> Result<&mut Type, CvError>
    {
        match self.op
        {
            Op::AddressOf => Err(CompilerError(MutTypeError("An address of unary operator ".to_owned()))),
            Op::Dereference =>
            {
                if !matches!(&*self.node.getType(), Type::Pointer(_))
                {
                    return Err(RuntimeError(InvalidDeref(self.node.getType().deref().clone())))
                }

                self.node.getTypeMut()
            },
            Op::Subscript(_) =>
            {
                if !matches!(&*self.node.getType(), Type::Array(_, _))
                {
                    return Err(RuntimeError(InvalidSubscript(self.node.getType().deref().clone())))
                }

                self.node.getTypeMut()
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