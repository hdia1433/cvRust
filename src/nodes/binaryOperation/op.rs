use std::fmt::Display;

#[derive(PartialEq)]
pub enum Op
{
    Assign,
}

impl Display for Op
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "=")
    }
}