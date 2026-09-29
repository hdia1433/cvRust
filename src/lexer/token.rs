use super::tokenType::TokenType;
use location::Location;

mod location;

#[derive(Debug)]
pub struct Token
{
    kind: TokenType,
    loc: Location
}

impl Token
{
    pub fn new(kind: TokenType, line: usize, column: usize) -> Self
    {
        Self {kind, loc: Location::new(line + 1, column + 1)}
    }
}
