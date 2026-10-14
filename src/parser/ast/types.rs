use std::fmt::Display;
use crate::lexer::token::TokenType;
use inkwell::{context::Context, types::{BasicMetadataTypeEnum, FunctionType, BasicTypeEnum}, AddressSpace};

#[derive(PartialEq, Clone, Default, Debug)]
pub enum Type
{
    Void,
    Int,
    Float,
    Bool,
    Char,
    Pointer(Box<Type>),
    #[default]
    Error
}

impl Type
{
    pub fn fromToken(tok: TokenType) -> Self
    {
        match tok
        {
            TokenType::KwBool => Self::Bool,
            TokenType::KwChar => Self::Char,
            TokenType::KwFloat => Self::Float,
            TokenType::KwInt => Self::Int,
            TokenType::KwVoid => Self::Void,
            _ => panic!("A token type for a type was needed")
        }
    }

    pub fn fnType<'a>(&self, context: &'a Context, args: &[BasicMetadataTypeEnum<'a>], isVarArgs: bool) -> FunctionType<'a>
    {
        match self
        {
            Type::Void => context.void_type().fn_type(args, isVarArgs),
            Type::Int => context.i32_type().fn_type(args, isVarArgs),
            Type::Float => context.f32_type().fn_type(args, isVarArgs),
            Type::Bool => context.bool_type().fn_type(args, isVarArgs),
            Type::Char => context.i8_type().fn_type(args, isVarArgs),
            Type::Pointer(_) => context.ptr_type(AddressSpace::default()).fn_type(args, isVarArgs),
            Type::Error => panic!("Cannot convert Type::Error into a function type")
        }
    }

    pub fn varType<'a>(&self, context: &'a Context) -> BasicTypeEnum<'a>
    {
        match self
        {
            Type::Int => context.i32_type().into(),
            Type::Float => context.f32_type().into(),
            Type::Bool => context.bool_type().into(),
            Type::Char => context.i8_type().into(),
            Type::Pointer(_) => context.ptr_type(AddressSpace::default()).into(),
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
            Type::Int => "<int>".to_string(),
            Type::Float => "<float>".to_string(),
            Type::Void => "<void>".to_string(),
            Type::Bool => "<bool>".to_string(),
            Type::Char => "<char>".to_string(),
            Type::Pointer(ptr) => format!("<{}*>", ptr),
            Type::Error => "<Error>".to_string()
        })?;

        Ok(())
    }
}