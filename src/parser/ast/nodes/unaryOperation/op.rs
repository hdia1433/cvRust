use std::fmt::Display;

#[derive(Debug)]
pub enum Op
{
    AddressOf,
    Dereference,
}

impl Display for Op
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{}", match self
        {
            Op::AddressOf => "&",
            Op::Dereference => "*",
        })
    }
}