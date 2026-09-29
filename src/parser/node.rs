pub use globalScope::GlobalScope;
pub use function::Function;
pub use variableDeclaration::VariableDeclaration;
pub use binaryOperation::BinaryOperation;
pub use literal::Literal;

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