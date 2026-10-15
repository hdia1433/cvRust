use std::fmt::Display;
use crate::parser::ast::nodes::Node;

#[derive(Debug)]
pub enum Op
{
    AddressOf,
    Dereference,
    Subscript(Box<Node>)
}

impl Display for Op
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{}", match self
        {
            Op::AddressOf => "&".to_string(),
            Op::Dereference => "*".to_string(),
            Op::Subscript(i) => format!("[{}]", i)
        })
    }
}