use crate::token::TokenType;
use crate::Type;
use super::Node;
use std::fmt::Display;

pub struct Function
{
    funcType: Type,
    name: String,
    body: Vec<Node>
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
            body: Vec::new()
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