use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Location
{
    line: usize,
    column: usize
}

impl Location
{
    pub fn new(line: usize, column: usize) -> Self
    {
        Self {line, column}
    }

    pub fn toString(&self) -> String
    {
        format!("the line {} and the column {}", self.line, self.column)
    }
}

impl Display for Location
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "the line {} and the column {}", self.line, self.column)
    }
}