pub use globalScope::GlobalScope;
pub use function::Function;
pub use variableDeclaration::VariableDeclaration;
pub use binaryOperation::BinaryOperation;
pub use literal::Literal;
use crate::lexer::tokenType::TokenType;

mod globalScope;
mod function;
mod variableDeclaration;
pub mod binaryOperation;
pub mod literal;

use std::fmt::Display;

pub enum Node
{
    VariableDeclaration(VariableDeclaration),
    BinaryOperation(BinaryOperation),
    Literal(Literal)
}

impl Node
{
    pub fn getType(&self) -> TokenType
    {
        match self
        {
            Node::VariableDeclaration(varDecl) => varDecl.getVarType().clone(),
            Node::BinaryOperation(binaryOp) => binaryOp.getType(),
            Node::Literal(literal) => literal.getType(),
        }
    }
}

impl Display for Node
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self 
        {
            Node::VariableDeclaration(varDecl) => writeln!(f, "{}", varDecl),
            Node::BinaryOperation(binaryOp) => writeln!(f, "{}", binaryOp),
            Node::Literal(literal) => writeln!(f, "{}", literal),
        }
    }
}