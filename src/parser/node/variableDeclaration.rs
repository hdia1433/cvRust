use crate::lexer::tokenType::TokenType;

pub struct VariableDeclaration
{
    varType: TokenType,
    name: String
}

impl VariableDeclaration
{
    pub fn new(varType: TokenType, name: &str) -> Self
    {
        Self {varType, name: name.to_owned()}
    }
}