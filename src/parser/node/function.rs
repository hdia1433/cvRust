use crate::lexer::tokenType::TokenType;
use super::Node;

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