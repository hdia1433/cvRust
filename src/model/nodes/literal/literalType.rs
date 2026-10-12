use crate::Type;
use inkwell::{values::BasicValueEnum, context::Context};
use std::fmt::Display;

#[derive(Debug)]
pub enum LiteralType
{
    Integer(i32),
    Float(f32),
    Bool(bool)
}

impl LiteralType
{
    pub fn getType(&self) -> Type
    {
        match self
        {
            LiteralType::Integer(_) => Type::Int,
            LiteralType::Float(_) => Type::Float,
            LiteralType::Bool(_) => Type::Bool,
        }
    }

    pub fn intoBasicValue<'a>(&self, context: &'a Context) -> BasicValueEnum<'a>
    {
        match self
        {
            LiteralType::Integer(integer) => context.i32_type().const_int(*integer as u64, true).into(),
            LiteralType::Float(float) => context.f32_type().const_float(*float as f64).into(),
            LiteralType::Bool(boolean) => context.bool_type().const_int(*boolean as u64, false).into()
        }
    }
}

impl Display for LiteralType
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self
        {
            LiteralType::Integer(integer) => write!(f, "Integer: {}", integer),
            LiteralType::Float(float) => write!(f, "Float: {}", float),
            LiteralType::Bool(boolean) => write!(f, "Boolean: {}", boolean)
        }
    }
}
