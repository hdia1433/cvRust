use crate::lexer::tokenType::TokenType;
use std::fmt::Display;

pub struct VariableDeclaration
{
    varType: TokenType,
    name: String
}

impl VariableDeclaration
{
    pub fn new(varType: TokenType, name: &str) -> Self
    {
        Self {varType, name: name.to_owned()}
    }

    pub fn getVarType(&self) -> &TokenType
    {
        &self.varType
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