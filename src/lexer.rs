pub mod tokenType;
pub mod token;

use token::{Token, location::Location};
use std::{iter::Peekable, str::Chars, fs::File, io::{Write, Error}};
use crate::lexer::tokenType::TokenType;

pub struct Lexer<'a>
{
    iter: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    line: usize,
    column: usize
}

impl<'a> Lexer<'a>
{
    pub fn new(source: &'a String) -> Self
    {
        Self {iter: source.chars().into_iter().peekable(), tokens: Vec::new(), line: 0, column: 0}
    }
}

impl Lexer<'_>
{
    pub fn tokenise(&mut self)
    {
        let mut buffer: String = String::new();
        let beginLoc = Location::new(self.line, self.column);

        while let Some(mut ch) = self.next()
        {
            buffer.clear();
            buffer.push(ch);
            if ch.is_alphabetic()
            {
                while let Some(next) = self.iter.peek() && next.is_alphanumeric()
                {
                    ch = self.next().expect("next character didn't exist");
                    buffer.push(ch);
                }

                self.tokens.push(Token::new(match buffer.as_str()
                {
                    "void" => TokenType::KwVoid,
                    "int" => TokenType::KwInt,
                    _ => TokenType::Identifier(buffer.clone())
                }, beginLoc.clone()));
            }
            else if ch.is_numeric()
            {
                while let Some(next) = self.iter.peek() && next.is_numeric()
                {
                    ch = self.next().expect("Next character didn't exist.");
                    buffer.push(ch);
                }

                buffer = dbg!(buffer);

                let integer: isize = buffer.parse().expect("Failed to convert buffer to isize");

                self.tokens.push(Token::new(TokenType::LitInteger(integer), beginLoc.clone()));
            }
            else 
            {
                self.tokens.push(Token::new(match ch
                {
                    '=' => TokenType::OpAssign,
                    '(' => TokenType::PuncOpenParen,
                    ')' => TokenType::PuncCloseParen,
                    '{' => TokenType::PuncOpenBrace,
                    '}' => TokenType::PuncCloseBrace,
                    ';' => TokenType::PuncSemi,
                    ' ' => continue,
                    '\n' => 
                    {
                        self.line += 1;
                        self.column = 0;
                        continue;
                    },
                    _ => panic!("Unrecognised token at the line {} and the column {}", self.line, self.column)
                }, beginLoc.clone()));
            }
        }

        println!("{:?}", self.tokens);
    }

    pub fn getTokens(&self) -> &Vec<Token>
    {
        &self.tokens
    }

    pub fn toFile(&self) -> Result<(), Error>
    {
        let mut file = File::create("tokens.txt")?;

        write!(file, "[")?;

        for token in &self.tokens
        {
            write!(file, "{}", token)?;
        }

        write!(file, "]")?;

        Ok(())
    }

    fn next(&mut self) -> Option<char>
    {
        self.column += 1;

        self.iter.next()
    }
}