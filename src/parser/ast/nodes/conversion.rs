use super::{Node, ToFile, Type};
use std::{fmt::Display, io::Write};

#[derive(Debug)]
pub struct Conversion
{
    node: Box<Node>,
    toType: Type
}

impl Conversion
{
    pub fn new(node: Node, toType: Type) -> Self
    {
        Self {node: Box::new(node), toType}
    }

    pub fn newFromBox(node: Box<Node>, toType: Type) -> Self
    {
        Self {node, toType}
    }

    pub fn getNode(&self) -> &Box<Node>
    {
        &self.node
    }

    pub fn getType(&self) -> Type
    {
        self.toType.clone()
    }

    pub fn getInitialType(&self) -> Type
    {
        self.node.getType()
    }
}

impl Display for Conversion
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "Convert {} to type {}", self.node, self.toType)
    }
}

impl ToFile for Conversion
{
    fn toFile(&self, f: &mut std::fs::File, indents: &mut bitvec::prelude::BitVec, last: bool) -> std::io::Result<()>
    {
        let indent:String = Self::getIndent(indents, last);

        write!(f, "{}Conversion(To: {})", indent, self.toType)?;

        indents.push(!last);

        self.node.toFile(f, indents, true)?;

        Ok(())
    }
}