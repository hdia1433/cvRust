use crate::Type;
use super::Node;
use std::fmt::Display;

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