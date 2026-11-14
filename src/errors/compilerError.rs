use thiserror::Error;

#[derive(Debug, Error)]
pub enum CompilerError
{
    #[error("{0} cannot return a &mut Type")]
    MutTypeError(String),
    #[error("An error node was found")]
    ErrorNodeFound
}