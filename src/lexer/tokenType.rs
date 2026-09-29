use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType
{
    KwVoid,
    KwInt,
    OpAssign,
    PuncOpenParen,
    PuncCloseParen,
    PuncOpenBrace,
    PuncCloseBrace,
    PuncSemi,
    Identifier(String),
    LitInteger(isize)
}

impl TokenType
{
    pub fn isType(&self) -> bool
    {
        *self == Self::KwVoid || *self == Self::KwInt
    }
}

impl Display for TokenType
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self
        {
            TokenType::KwVoid => write!(f, "void"),
            TokenType::KwInt => write!(f, "int"),
            TokenType::OpAssign => write!(f, "OP_ASSIGN"),
            TokenType::PuncOpenParen => write!(f, "OPEN_PAREN"),
            TokenType::PuncCloseParen => write!(f, "CLOSE_PAREN"),
            TokenType::PuncOpenBrace => write!(f, "OPEN_BRACE"),
            TokenType::PuncCloseBrace => write!(f, "CLOSE_BRACE"),
            TokenType::PuncSemi => write!(f, "SEMI_COLON"),
            TokenType::Identifier(name) => write!(f, "IDENTIFIER: {}", name),
            TokenType::LitInteger(integer) => write!(f, "INTEGER_LITERAL: {}", integer)
        }
    }
}