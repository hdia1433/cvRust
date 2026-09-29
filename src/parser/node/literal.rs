pub enum LiteralType
{
    Integer(isize)
}

pub struct Literal
{
    value: LiteralType
}

impl Literal
{
    pub fn new(value: LiteralType) -> Self
    {
        Self {value}
    }
}

