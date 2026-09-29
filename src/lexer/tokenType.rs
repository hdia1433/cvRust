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
    Integer(isize)
}

impl TokenType
{
    pub fn isType(&self) -> bool
    {
        *self == Self::KwVoid || *self == Self::KwInt
    }
}