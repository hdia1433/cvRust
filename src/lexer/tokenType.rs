#[derive(Debug)]
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