use std::{fmt::Display, io::Write};

use super::{VariableDeclaration, VariableAccess, BinaryOperation, Conversion, Literal, Type, ToFile};

#[derive(Default, Debug)]
pub enum Node
{
    VariableDeclaration(VariableDeclaration),
    VariableAccess(VariableAccess),
    BinaryOperation(BinaryOperation),
    Conversion(Conversion),
    Literal(Literal),
    #[default]
    Error
}

impl Node
{
    pub fn getType(&self) -> Type
    {
        match self
        {
            Node::VariableDeclaration(varDecl) => varDecl.getVarType().clone(),
            Node::VariableAccess(varAccess) => varAccess.getVarType().clone(),
            Node::BinaryOperation(binaryOp) => binaryOp.getType(),
            Node::Conversion(conversion) => conversion.getType(),
            Node::Literal(literal) => literal.getType(),
            Node::Error => panic!("Cannot get the type of an error node")
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
            Node::Literal(literal) => writeln!(f, "{}", literal),
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
            Node::Error => 
            {
                let indent:String = Self::getIndent(indents, last);

                write!(f, "{}Error", indent)
            },
        }
    }
}