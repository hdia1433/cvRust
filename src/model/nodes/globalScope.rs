use super::{Function, ToFile};
use std::{fmt::Display, io::Write};
use bitvec::prelude::BitVec;

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

    pub fn getFunctions(&self) -> &Vec<Function>
    {
        &self.functions
    }

    pub fn getFunctionsMut(&mut self) -> &mut Vec<Function>
    {
        &mut self.functions
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

impl ToFile for GlobalScope
{
    fn toFile(&self, f: &mut std::fs::File, indents: &mut BitVec, last: bool) -> std::io::Result<()> 
    {
        let indent:String = Self::getIndent(indents, last);

        write!(f, "{}Global Scope", indent)?;

        let mut childIndents:BitVec = indents.clone();

        childIndents.push(!last);

        for (i, func) in self.functions.iter().enumerate()
        {
            func.toFile(f, &mut childIndents, i == self.functions.len() - 1)?;
        }

        Ok(())
    }
}