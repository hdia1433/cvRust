mod tokenType;
mod token;

use token::Token;
use std::{iter::Peekable, str::Chars};
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
                }, self.line, self.column));
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

                self.tokens.push(Token::new(TokenType::Integer(integer), self.line, self.column));
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
                }, self.line, self.column));
            }
        }

        println!("{:?}", self.tokens);
    }

    pub fn getTokens(&self) -> &Vec<Token>
    {
        &self.tokens
    }

    fn next(&mut self) -> Option<char>
    {
        self.column += 1;

        self.iter.next()
    }
}