use crate::lexer::tokenType::TokenType;
use super::Node;
use std::fmt::Display;

pub struct Function
{
    funcType: TokenType,
    name: String,
    body: Vec<Node>
}

impl Function
{
    pub fn new(funcType: TokenType, name: &str) -> Self
    {
        Self {funcType, name: name.to_owned(), body: Vec::new()}
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