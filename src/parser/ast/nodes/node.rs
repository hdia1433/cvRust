use std::{fmt::Display, io::Write, borrow::Cow};

use super::{VariableDeclaration, VariableAccess, BinaryOperation, Conversion, Literal, Type, ToFile, UnaryOperation};
use crate::errors::{CompilerError::{MutTypeError, ErrorNodeFound}, CvError::{self, CompilerError}};

#[derive(Default, Debug, Clone)]
pub enum Node
{
    VariableDeclaration(VariableDeclaration),
    VariableAccess(VariableAccess),
    BinaryOperation(BinaryOperation),
    Conversion(Conversion),
    Literal(Literal),
    UnaryOperation(UnaryOperation),
    #[default]
    Error
}

impl Node
{
    pub fn getType(&self) -> Cow<'_, Type>
    {
        match self
        {
            Node::VariableDeclaration(varDecl) => Cow::Borrowed(varDecl.getVarType()),
            Node::VariableAccess(varAccess) => Cow::Borrowed(varAccess.getVarType()),
            Node::BinaryOperation(binaryOp) => binaryOp.getType(),
            Node::Conversion(conversion) => Cow::Borrowed(conversion.getType()),
            Node::Literal(literal) => literal.getType(),
            Node::UnaryOperation(addressOf) => addressOf.getType(),
            Node::Error => panic!("Cannot get the type of an error node")
        }
    }

    pub fn getTypeMut(&mut self) -> Result<&mut Type, CvError>
    {
        match self
        {
            Node::VariableDeclaration(varDecl) => Ok(varDecl.getVarTypeMut()),
            Node::VariableAccess(varAccess) => Ok(varAccess.getVarTypeMut()),
            Node::BinaryOperation(binaryOp) => binaryOp.getTypeMut(),
            Node::Conversion(conversion) => Ok(conversion.getTypeMut()),
            Node::Literal(_) => Err(CompilerError(MutTypeError("A literal ".to_owned()))),
            Node::UnaryOperation(unaryOp) => unaryOp.getTypeMut(),
            Node::Error => Err(CompilerError(ErrorNodeFound)),
        }
    }
}

impl Display for Node
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self 
        {
            Node::VariableDeclaration(varDecl) => writeln!(f, "{}", varDecl),
            Node::VariableAccess(varAccess) => writeln!(f, "{}", varAccess),
            Node::BinaryOperation(binaryOp) => writeln!(f, "{}", binaryOp),
            Node::Conversion(conversion) => write!(f, "{}", conversion),
            Node::Literal(literal) => write!(f, "{}", literal),
            Node::UnaryOperation(addressOf) => write!(f, "{}", addressOf),
            Node::Error => write!(f, "error")
        }
    }
}

impl ToFile for Node
{
    fn toFile(&self, f: &mut std::fs::File, indents: &mut bitvec::prelude::BitVec, last: bool) -> std::io::Result<()>
    {
        match self
        {
            Node::VariableDeclaration(varDecl) => varDecl.toFile(f, indents, last),
            Node::VariableAccess(varAccess) => varAccess.toFile(f, indents, last),
            Node::BinaryOperation(binOp) => binOp.toFile(f, indents, last),
            Node::Conversion(conversion) => conversion.toFile(f, indents, last),
            Node::Literal(literal) => literal.toFile(f, indents, last),
            Node::UnaryOperation(addressOf) => addressOf.toFile(f, indents, last),
            Node::Error => 
            {
                let indent:String = Self::getIndent(indents, last);

                write!(f, "{}Error", indent)
            },
        }
    }
}