use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType
{
    KwVoid,
    KwInt,
    KwFloat,
    KwBool,
    KwChar,
    OpAssign,
    OpPlus,
    OpMinus,
    OpStar,
    OpSlash,
    OpEquals,
    PuncOpenParen,
    PuncCloseParen,
    PuncOpenBrace,
    PuncCloseBrace,
    PuncSemi,
    Identifier(String),
    LitInteger(i32),
    LitFloat(f32),
    LitBool(bool),
    LitChar(char)
}

impl TokenType
{
    pub fn isType(&self) -> bool
    {
        *self == Self::KwVoid || *self == Self::KwInt || *self == Self::KwFloat || *self == Self::KwBool || *self == Self::KwChar
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
            TokenType::KwChar => write!(f, "KW: char"),
            TokenType::OpAssign => write!(f, "OP_ASSIGN"),
            TokenType::OpPlus => write!(f, "OP_PLUS"),
            TokenType::OpMinus => write!(f, "OP_MINUS"),
            TokenType::OpStar => write!(f, "OP_STAR"),
            TokenType::OpSlash => write!(f, "OP_SLASH"),
            TokenType::OpEquals => write!(f, "OP_EQUALS"),
            TokenType::PuncOpenParen => write!(f, "OPEN_PAREN"),
            TokenType::PuncCloseParen => write!(f, "CLOSE_PAREN"),
            TokenType::PuncOpenBrace => write!(f, "OPEN_BRACE"),
            TokenType::PuncCloseBrace => write!(f, "CLOSE_BRACE"),
            TokenType::PuncSemi => write!(f, "SEMI_COLON"),
            TokenType::Identifier(name) => write!(f, "IDENT: {}", name),
            TokenType::LitInteger(integer) => write!(f, "INT_LITERAL: {}", integer),
            TokenType::LitFloat(float) => write!(f, "FLOAT_LITERAL: {}", float),
            TokenType::LitBool(bool) => write!(f, "BOOL_LITERAL: {}", bool),
            TokenType::LitChar(char) => write!(f, "CHAR_LITERAL: {}", char)
        }
    }
}