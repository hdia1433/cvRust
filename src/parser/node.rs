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

pub enum Node
{
    VariableDeclaration(VariableDeclaration),
    BinaryOperation(BinaryOperation),
    Literal(Literal)
}