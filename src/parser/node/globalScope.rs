use super::Function;
use crate::lexer::token::Token;
use std::{iter::Peekable, slice::Iter};

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