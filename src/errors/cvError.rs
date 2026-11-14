use thiserror::Error;
use super::{CompilerError, RuntimeError};

pub type CvResult<T> = Result<T, CvError>;

#[derive(Error, Debug)]
pub enum CvError
{
    #[error("CompilerError: {0}")]
    CompilerError(CompilerError),
    #[error("RuntimeError: {0}")]
    RuntimeError(RuntimeError)
}