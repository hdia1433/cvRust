use crate::{Type, token::TokenType};
use std::fmt::Display;

#[derive(Clone)]
pub struct VariableDeclaration
{
    varType: Type,
    name: String
}

impl VariableDeclaration
{
    pub fn new(varType: TokenType, name: &str) -> Self
    {
        Self 
        {
            varType: match varType
            {
                TokenType::KwInt => Type::Int,
                TokenType::KwVoid => Type::Void,
                _ => panic!("A token type of a type required to create a new variable declaration")
            }, 
            name: name.to_owned()
        }
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