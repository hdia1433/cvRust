use crate::{parser::ast::{ToFile, Type, nodes::Node}};
use bitvec::vec::BitVec;
use anyhow::{Ok, Result as AResult};
use inkwell::{builder::Builder, context::Context, types::BasicType, values::{ArrayValue, BasicValueEnum}};
use std::{fmt::Display, io::Write, ops::Deref, borrow::Cow};

#[derive(Debug, Clone)]
pub enum LiteralType
{
    Integer(i32),
    Float(f32),
    Bool(bool),
    Char(char),
    Array(Vec<Node>)
}

impl LiteralType
{
    pub fn getType(&self) -> Cow<'_, Type>
    {
        match self
        {
            LiteralType::Integer(_) => Cow::Owned(Type::Int),
            LiteralType::Float(_) => Cow::Owned(Type::Float),
            LiteralType::Bool(_) => Cow::Owned(Type::Bool),
            LiteralType::Char(_) => Cow::Owned(Type::Char),
            LiteralType::Array(arr) => Cow::Owned(Type::Array(arr.len() as u32, Box::new(arr[0].getType().deref().clone())))
        }
    }

    pub fn intoBasicValue<'a>(&self, context: &'a Context, builder: &Builder<'a>) -> AResult<BasicValueEnum<'a>>
    {
        match self
        {
            LiteralType::Integer(integer) => Ok(context.i32_type().const_int(*integer as u64, true).into()),
            LiteralType::Float(float) => Ok(context.f32_type().const_float(*float as f64).into()),
            LiteralType::Bool(boolean) => Ok(context.bool_type().const_int(*boolean as u64, false).into()),
            LiteralType::Char(character) => Ok(context.i8_type().const_int(*character as u64, false).into()),
            LiteralType::Array(values) => 
            {
                let mut arr: ArrayValue = values.first().expect("Failed to get first element").getType().varType(context).array_type(values.len() as u32).get_undef();
                
                for i in 0..values.len()
                {
                    if let Node::Literal(literal) = &values[i]
                    {
                        arr = builder.build_insert_value(arr, literal.getValue().intoBasicValue(context, builder)?, i as u32, "Insert")?.into_array_value();
                    }
                }

                Ok(arr.into())
            }
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
            LiteralType::Bool(boolean) => write!(f, "Boolean: {}", boolean),
            LiteralType::Char(character) => write!(f, "Character: \'{}\'", character),
            LiteralType::Array(arr) => {write!(f, "Array: {:?}", arr)}
        }
    }
}

impl ToFile for LiteralType
{
    fn toFile(&self, f: &mut std::fs::File, indents: &mut BitVec, last: bool) -> std::io::Result<()> 
    {
        let indent: String = Self::getIndent(indents, last);
        write!(f, "{}", indent)?;
        match self
        {
            LiteralType::Integer(integer) => write!(f, "Integer: {}", integer),
            LiteralType::Float(float) => write!(f, "Float: {}", float),
            LiteralType::Bool(boolean) => write!(f, "Boolean: {}", boolean),
            LiteralType::Char(character) => write!(f, "Character: \'{}\'", character),
            LiteralType::Array(arr) =>
            {
                write!(f, "Array:")?;

                let mut innerIndents: BitVec = indents.clone();
                innerIndents.push(!last);

                for i in 0..arr.len()
                {
                    arr[i].toFile(f, &mut innerIndents, i == arr.len() - 1)?;
                };

                std::io::Result::Ok(())
            }
        }
    }
}
