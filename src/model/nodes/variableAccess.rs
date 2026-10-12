use crate::Type;
use std::{fmt::Display, io::Write};
use super::ToFile;

#[derive(Debug)]
pub struct VariableAccess
{
    name: String,
    varType: Type
}

impl VariableAccess
{
    pub fn new(name: &str) -> Self
    {
        Self {name: name.to_owned(), varType: Type::default()}
    }

    pub fn getName(&self) -> &String
    {
        &self.name
    }

    pub fn getVarType(&self) -> &Type
    {
        &self.varType
    }

    pub fn setVarType(&mut self, varType: Type)
    {
        self.varType = varType;
    }
}

impl Display for VariableAccess
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        writeln!(f, "VariableAccess ({})", self.name)
    }
}

impl ToFile for VariableAccess
{
    fn toFile(&self, f: &mut std::fs::File, indents: &mut bitvec::prelude::BitVec, last: bool) -> std::io::Result<()> 
    {
        let indent: String = Self::getIndent(indents, last);

        write!(f, "{}Variable Access({}, {})", indent, self.varType, self.name)
    }
}