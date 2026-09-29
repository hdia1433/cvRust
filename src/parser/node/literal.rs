use std::fmt::Display;

pub enum LiteralType
{
    Integer(isize)
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
}

impl Display for Literal
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        writeln!(f, "Literal: {}", self.value)
    }
}

