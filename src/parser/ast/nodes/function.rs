use crate::lexer::token::TokenType;
use super::{Node, ToFile, Type};
use std::{fmt::Display, io::Write};
use bitvec::prelude::BitVec;

pub struct Function
{
    funcType: Type,
    name: String,
    body: Vec<Node>,
    noReturn: bool
}

impl Function
{
    pub fn new(funcType: TokenType, name: &str) -> Self
    {
        Self 
        {
            funcType: match funcType
            {
                TokenType::KwInt => Type::Int,
                TokenType::KwVoid => Type::Void,
                _ => panic!("A token type of a type is needed to create a function")
            }, 
            name: name.to_owned(), 
            body: Vec::new(),
            noReturn: false
        }
    }

    pub fn getType(&self) -> &Type
    {
        &self.funcType
    }

    pub fn getName(&self) -> &String
    {
        &self.name
    }

    pub fn getBody(&self) -> &Vec<Node>
    {
        &self.body
    }

    pub fn getBodyMut(&mut self) -> &mut Vec<Node>
    {
        &mut self.body
    }

    pub fn getNoReturn(&self) -> &bool
    {
        &self.noReturn
    }

    pub fn setRoReturn(&mut self, noReturn: bool)
    {
        self.noReturn = noReturn;
    }

    pub fn addStatement(&mut self, node: Node)
    {
        self.body.push(node);
    }
}

impl Display for Function
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        writeln!(f, "Function({} {})\n{{", self.funcType, self.name)?;

        for node in &self.body
        {
            writeln!(f, "{}", node)?;
        }

        writeln!(f, "}}")?;

        Ok(())
    }
}

impl ToFile for Function
{
    fn toFile(&self, f: &mut std::fs::File, indents: &mut BitVec, last: bool) -> std::io::Result<()> 
    {
        let indent:String = Self::getIndent(indents, last);

        write!(f, "{}Function({}, {})", indent, self.funcType, self.name)?;

        let mut childIndents:BitVec = indents.clone();

        childIndents.push(!last);

        for (i,node) in self.body.iter().enumerate()
        {
            node.toFile(f, &mut childIndents, i == self.body.len() - 1)?;
        }

        Ok(())
    }
}