use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType
{
    KwVoid,
    KwInt,
    KwFloat,
    KwBool,
    KwTrue,
    KwFalse,
    OpAssign,
    OpPlus,
    OpMinus,
    OpStar,
    OpSlash,
    PuncOpenParen,
    PuncCloseParen,
    PuncOpenBrace,
    PuncCloseBrace,
    PuncSemi,
    Identifier(String),
    LitInteger(i32),
    LitFloat(f32)
}

impl TokenType
{
    pub fn isType(&self) -> bool
    {
        *self == Self::KwVoid || *self == Self::KwInt || *self == Self::KwFloat
    }
}

impl Display for TokenType
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        match self
        {
            TokenType::KwVoid => write!(f, "KW: void"),
            TokenType::KwInt => write!(f, "KW: int"),
            TokenType::KwFloat => write!(f, "KW: float"),
            TokenType::KwBool => write!(f, "KW: bool"),
            TokenType::KwTrue => write!(f, "KW: true"),
            TokenType::KwFalse => write!(f, "KW: false"),
            TokenType::OpAssign => write!(f, "OP_ASSIGN"),
            TokenType::OpPlus => write!(f, "OP_PLUS"),
            TokenType::OpMinus => write!(f, "OP_MINUS"),
            TokenType::OpStar => write!(f, "OP_STAR"),
            TokenType::OpSlash => write!(f, "OP_SLASH"),
            TokenType::PuncOpenParen => write!(f, "OPEN_PAREN"),
            TokenType::PuncCloseParen => write!(f, "CLOSE_PAREN"),
            TokenType::PuncOpenBrace => write!(f, "OPEN_BRACE"),
            TokenType::PuncCloseBrace => write!(f, "CLOSE_BRACE"),
            TokenType::PuncSemi => write!(f, "SEMI_COLON"),
            TokenType::Identifier(name) => write!(f, "IDENT: {}", name),
            TokenType::LitInteger(integer) => write!(f, "INT_LITERAL: {}", integer),
            TokenType::LitFloat(float) => write!(f, "FLOAT_LITERAL: {}", float)
        }
    }
}