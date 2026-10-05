pub use globalScope::GlobalScope;
pub use function::Function;
pub use variableDeclaration::VariableDeclaration;
pub use variableAccess::VariableAccess;
pub use binaryOperation::BinaryOperation;
pub use literal::Literal;
use crate::Type;

mod globalScope;
mod function;
mod variableDeclaration;
mod variableAccess;
pub mod binaryOperation;
pub mod literal;

use std::fmt::Display;

pub enum Node
{
    VariableDeclaration(VariableDeclaration),
    VariableAccess(VariableAccess),
    BinaryOperation(BinaryOperation),
    Literal(Literal)
}

impl Node
{
    pub fn getType(&self) -> Type
    {
        match self
        {
            Node::VariableDeclaration(varDecl) => varDecl.getVarType().clone(),
            Node::VariableAccess(varAccess) => varAccess.getVarType().clone(),
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
            Node::VariableAccess(varAccess) => writeln!(f, "{}", varAccess),
            Node::BinaryOperation(binaryOp) => writeln!(f, "{}", binaryOp),
            Node::Literal(literal) => writeln!(f, "{}", literal),
        }
    }
}