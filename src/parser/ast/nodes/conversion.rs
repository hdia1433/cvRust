use super::{Node, ToFile, Type};
use std::{fmt::Display, io::Write, borrow::Cow};

#[derive(Debug, Clone)]
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

    pub fn getType(&self) -> &Type
    {
        &self.toType
    }

    pub fn getTypeMut(&mut self) -> &mut Type
    {
        &mut self.toType
    }

    pub fn getInitialType(&self) -> Cow<'_, Type>
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