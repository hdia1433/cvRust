use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub enum Op
{
    Assign,
    Add,
    Sub,
    Mul,
    Div
}

impl Display for Op
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{}", match self
        {
            Op::Assign => "=",
            Op::Add => "+",
            Op::Sub => "-",
            Op::Mul => "*",
            Op::Div => "/"
        })
    }
}