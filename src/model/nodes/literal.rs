use std::fmt::Display;
use crate::Type;
use inkwell::{values::BasicValueEnum, context::Context};

#[derive(Debug)]
pub enum LiteralType
{
    Integer(i32),
    Float(f32)
}

impl LiteralType
{
    pub fn getType(&self) -> Type
    {
        match self
        {
            LiteralType::Integer(_) => Type::Int,
            LiteralType::Float(_) => Type::Float
        }
    }

    pub fn intoBasicValue<'a>(&self, context: &'a Context) -> BasicValueEnum<'a>
    {
        match self
        {
            LiteralType::Integer(integer) => context.i32_type().const_int(*integer as u64, true).into(),
            LiteralType::Float(float) => context.f32_type().const_float(*float as f64).into()
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
            LiteralType::Float(float) => write!(f, "Float: {}", float)
        }
    }
}

#[derive(Debug)]
pub struct Literal
{
    value: LiteralType
}

impl Literal
{
    pub fn new(value: LiteralType) -> Self
    {
        Self {value}
    }

    pub fn getValue(&self) -> &LiteralType
    {
       &self.value
    }

    pub fn getType(&self) -> Type
    {
        self.value.getType()
    }
}

impl Display for Literal
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        writeln!(f, "Literal: {}", self.value)
    }
}

