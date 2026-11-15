use std::{borrow::Cow, fmt::Display, io::Write};
use bitvec::vec::BitVec;

use super::LiteralType;
use crate::parser::ast::{ToFile, Type};


#[derive(Debug, Clone)]
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

    pub fn getValueMut(&mut self) -> &mut LiteralType
    {
        &mut self.value
    }

    pub fn getType(&self) -> Cow<'_, Type>
    {
        self.value.getType()
    }
}

impl Display for Literal
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "Literal: {}", self.value)
    }
}

impl ToFile for Literal
{
    fn toFile(&self, f: &mut std::fs::File, indents: &mut bitvec::prelude::BitVec, last: bool) -> std::io::Result<()>
    {
        let indent:String = Self::getIndent(indents, last);

        write!(f, "{}Literal:", indent)?;
        let mut innerIndents: BitVec = indents.clone();
        innerIndents.push(!last);
        self.value.toFile(f, &mut innerIndents, true)
    }
}