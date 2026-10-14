use super::{VariableAccess, Type};
use super::ToFile;
use bitvec::prelude::BitVec;

use std::{io::Write, fmt::Display};

#[derive(Debug)]
pub struct AddressOf
{
    var: VariableAccess
}

impl AddressOf
{
    pub fn new(var: VariableAccess) -> Self
    {
        Self {var}
    }

    pub fn getType(&self) -> Type
    {
        Type::Pointer(Box::new(self.var.getType()))
    }
}

impl Display for AddressOf
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "AddressOf: {}", self.var)
    }
}

impl ToFile for AddressOf
{
    fn toFile(&self, f: &mut std::fs::File, indents: &mut BitVec, last: bool) -> std::io::Result<()> 
    {
        let indent:String = Self::getIndent(indents, last);

        write!(f, "{}Address Of", indent)?;

        let mut childIndents:BitVec = indents.clone();
        childIndents.push(!last);

        self.var.toFile(f, &mut childIndents, true)?;

        Ok(())
    }
}