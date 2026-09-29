use super::Function;
use std::fmt::Display;

pub struct GlobalScope
{
    functions: Vec<Function>
}

impl GlobalScope
{
    pub fn new() -> Self
    {
        Self{functions: Vec::new()}
    }

    pub fn addFunction(&mut self, function: Function)
    {
        self.functions.push(function);
    }
}

impl Display for GlobalScope
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        writeln!(f, "GlobalScope\n{{")?;

        for func in &self.functions
        {
            writeln!(f, "{}", func)?;
        }

        writeln!(f, "}}")?;

        Ok(())
    }
}