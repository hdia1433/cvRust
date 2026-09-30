use std::fmt::Display;
use crate::lexer::tokenType::TokenType;

pub enum LiteralType
{
    Integer(isize)
}

impl LiteralType
{
    pub fn getType(&self) -> TokenType
    {
        match self
        {
            LiteralType::Integer(_) => TokenType::KwInt
        }
    }
}

impl Display for LiteralType
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self
        {
            LiteralType::Integer(integer) => write!(f, "Integer: {}", integer)
        }
    }
}

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

    pub fn getType(&self) -> TokenType
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

