use std::{fmt::Display, io::Write};
use super::{ToFile, Type};

#[derive(Clone, Debug)]
pub struct VariableDeclaration
{
    varType: Type,
    name: String
}

impl VariableDeclaration
{
    pub fn new(varType: Type, name: &str) -> Self
    {
        Self {varType, name: name.to_owned()}
    }

    pub fn getVarType(&self) -> &Type
    {
        &self.varType
    }

    pub fn getName(&self) -> &str
    {
        &self.name
    }
}

impl Display for VariableDeclaration
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        writeln!(f, "VariableDeclaration")?;
        writeln!(f, "{{")?;
        writeln!(f, "Type: {}", self.varType)?;
        writeln!(f, "Name: {}", self.name)?;
        writeln!(f, "}}")?;

        Ok(())
    }
}

impl ToFile for VariableDeclaration
{
    fn toFile(&self, f: &mut std::fs::File, indents: &mut bitvec::prelude::BitVec, last: bool) -> std::io::Result<()>
    {
        let indent:String = Self::getIndent(indents, last);

        write!(f, "{}Variable Declaration({}, {})", indent, self.varType, self.name)
    }
}