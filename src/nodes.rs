pub use globalScope::GlobalScope;
pub use function::Function;
pub use variableDeclaration::VariableDeclaration;
pub use variableAccess::VariableAccess;
pub use binaryOperation::BinaryOperation;
pub use conversion::Conversion;
pub use literal::Literal;
use crate::Type;

mod globalScope;
mod function;
mod variableDeclaration;
mod variableAccess;
pub mod binaryOperation;
mod conversion;
pub mod literal;

use std::fmt::Display;

#[derive(Default, Debug)]
pub enum Node
{
    VariableDeclaration(VariableDeclaration),
    VariableAccess(VariableAccess),
    BinaryOperation(BinaryOperation),
    Conversion(Conversion),
    Literal(Literal),
    #[default]
    Error
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
            Node::Conversion(conversion) => conversion.getType(),
            Node::Literal(literal) => literal.getType(),
            Node::Error => panic!("Cannot get the type of an error node")
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
            Node::Conversion(conversion) => write!(f, "{}", conversion),
            Node::Literal(literal) => writeln!(f, "{}", literal),
            Node::Error => write!(f, "error")
        }
    }
}