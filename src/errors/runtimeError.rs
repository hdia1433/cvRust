use thiserror::Error;

use crate::parser::ast::Type;

#[derive(Debug, Error)]
pub enum RuntimeError
{
    #[error("Type {0} cannot be dereferenced")]
    InvalidDeref(Type),
    #[error("The subscript operator ([]) cannot be applied to the type {0}")]
    InvalidSubscript(Type),
    #[error("There is no conversion between {0} and {1}")]
    InvalidConversion(Type, Type),
    #[error("A subscript operator cannot be used on a variable that is not of an array type")]
    SubscriptOnNonArray
}