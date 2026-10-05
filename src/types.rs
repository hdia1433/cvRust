use std::fmt::Display;
use inkwell::{context::Context, types::{BasicMetadataTypeEnum, FunctionType, BasicTypeEnum}};

#[derive(PartialEq, Clone, Default)]
pub enum Type
{
    Void,
    Int,
    #[default]
    Error
}

impl Type
{
    pub fn fnType<'a>(&self, context: &'a Context, args: &[BasicMetadataTypeEnum<'a>], isVarArgs: bool) -> FunctionType<'a>
    {
        match self
        {
            Type::Void => context.void_type().fn_type(args, isVarArgs),
            Type::Int => context.i32_type().fn_type(args, isVarArgs),
            Type::Error => panic!("Cannot convert Type::Error into a function type")
        }
    }

    pub fn varType<'a>(&self, context: &'a Context) -> BasicTypeEnum<'a>
    {
        match self
        {
            Type::Int => context.i32_type().into(),
            Type::Void => panic!("Variables cannot be of the type <void>"),
            Type::Error => panic!("Type::Error couldn't be converted into variable type")
        }
    }
}

impl Display for Type
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "{}",
        match self
        {
            Type::Int => "<int>",
            Type::Void => "<void>",
            Type::Error => "<Error>"
        })?;

        Ok(())
    }
}