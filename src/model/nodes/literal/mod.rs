mod literalType;

use std::{fmt::Display, io::Write};
use crate::Type;
pub use literalType::LiteralType;
use super::ToFile;


#[derive(Debug)]
pub struct Literal
{
    value: LiteralType
}

impl Literal
{
    pub fn new(value: LiteralType) -> Self
    {
        Self {value}
    }

    pub fn getValue(&self) -> &LiteralType
    {
       &self.value
    }

    pub fn getType(&self) -> Type
    {
        self.value.getType()
    }
}

impl Display for Literal
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        writeln!(f, "Literal: {}", self.value)
    }
}

impl ToFile for Literal
{
    fn toFile(&self, f: &mut std::fs::File, indents: &mut bitvec::prelude::BitVec, last: bool) -> std::io::Result<()>
    {
        let indent:String = Self::getIndent(indents, last);

        write!(f, "{}Literal({})", indent, self.value)
    }
}