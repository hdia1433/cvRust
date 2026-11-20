use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompilerError
{
    #[error("{0} cannot return a &mut Type")]
    MutTypeError(String),
    #[error("An error node was found")]
    ErrorNodeFound,
    #[error("A basic value was expected, but one wasn't provided")]
    ExpectedBasicValue,
    #[error("A pointer value was expected, but one wasn't provided")]
    ExpectedPointerValue,
    #[error("Expected {0} node")]
    ExpectedNode(String)
}