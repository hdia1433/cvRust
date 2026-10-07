use crate::token::{Location, Token, TokenType};
use std::{iter::Peekable, str::Chars, fs::File, io::{Write, Error}};

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
        Self {iter: source.chars().into_iter().peekable(), tokens: Vec::new(), line: 1, column: 0}
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
                    "float" => TokenType::KwFloat,
                    _ => TokenType::Identifier(buffer.clone())
                }, beginLoc.clone()));
            }
            else if ch.is_numeric()
            {
                let mut dotNum:i32 = 0;
                while let Some(next) = self.iter.peek() && (next.is_numeric() || *next == '.')
                {
                    ch = self.next().expect("Next character didn't exist.");
                    if ch == '.'
                    {
                        dotNum += 1;
                    }
                    buffer.push(ch);
                }

                let tok:Token = match dotNum
                {
                    0 =>
                    {
                        let integer: i32 = buffer.parse().expect("Failed to convert buffer to i32");
                        Token::new(TokenType::LitInteger(integer), beginLoc.clone())
                    },
                    1 =>
                    {
                        let float:f32 = buffer.parse().expect("Failed to convert buffer to f32");
                        Token::new(TokenType::LitFloat(float), beginLoc.clone())
                    },
                    _ => panic!("A floating point value can only contain 1 '.'")
                };

                

                self.tokens.push(tok);
            }
            else 
            {
                self.tokens.push(Token::new(match ch
                {
                    '=' => TokenType::OpAssign,
                    '+' => TokenType::OpPlus,
                    '-' => TokenType::OpMinus,
                    '*' => TokenType::OpStar,
                    '(' => TokenType::PuncOpenParen,
                    ')' => TokenType::PuncCloseParen,
                    '{' => TokenType::PuncOpenBrace,
                    '}' => TokenType::PuncCloseBrace,
                    ';' => TokenType::PuncSemi,
                    ' ' => continue,
                    '/' => 
                    {
                        let Some(next) = self.iter.peek() else
                        {
                            continue;
                        };
                        if *next == '/'
                        {
                            while let Some(ch) = self.next() && ch != '\n'
                            {
                                   
                            }
                            continue;
                        }
                        else if *next == '*'
                        {
                            loop
                            {
                                let Some(ch) = self.next() else
                                {
                                    panic!("An error has occurred at the end of the file. A '*/' is needed to end a multiline comment");
                                };
                                let Some(next) = self.iter.peek() else
                                {
                                    panic!("An error has occurred at the end of the file. A '*/' is needed to end a multiline comment");
                                };

                                if ch == '*' && *next == '/'
                                {
                                    self.next();
                                    break;
                                }
                            }
                            continue;
                        }
                        continue;
                    },
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