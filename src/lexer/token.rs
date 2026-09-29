use super::tokenType::TokenType;
use location::Location;
use std::fmt::Display;

pub mod location;

#[derive(Debug)]
pub struct Token
{
    kind: TokenType,
    loc: Location
}

impl Token
{
    pub fn new(kind: TokenType, loc: Location) -> Self
    {
        Self {kind, loc}
    }

    pub fn getKind(&self) -> &TokenType
    {
        &self.kind
    }

    pub fn getLoc(&self) -> &Location
    {
        &self.loc
    }
}

impl Display for Token
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{{{}}}", self.getKind())
    }
}
