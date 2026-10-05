use crate::token::TokenType;
use crate::Type;
use super::Node;
use std::fmt::Display;

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

    pub fn getName(&self) -> &str
    {
        &self.name
    }

    pub fn getBody(&self) -> &Vec<Node>
    {
        &self.body
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