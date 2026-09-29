use super::lexer::{token::Token, tokenType::TokenType};
use node::{GlobalScope, Node, Function};
use std::{iter::Peekable, slice::Iter};

mod node;

pub struct Parser<'a>
{
    iter: Peekable<Iter<'a, Token>>,
    ast: GlobalScope
}

impl<'a> Parser<'a>
{
    pub fn new(tokens: &'a Vec<Token>) -> Self
    {
        let iter = tokens.into_iter().peekable();

        Self {iter, ast: GlobalScope::new()}
    }

    pub fn parse(&mut self)
    {
        while let Some(tok) = self.iter.next()
        {
            if !tok.getKind().isType()
            {
                panic!("An error has occurred at {}. A function declaration must begin with a type.", tok.getLoc().toString());
            }

            let funcType = tok.getKind().clone();

            let Some(tok) = self.iter.next() else
            {
                panic!("An error has occurred at the end of the file. A function declaration must contain a name after the type.");
            };

            let TokenType::Identifier(funcName) = tok.getKind() else
            {
                panic!("An error has occurred at {}. A function declaration must contain a name after the type.", tok.getLoc());
            };

            let Some(tok) = self.iter.next() else
            {
                panic!("An error has occurred at the end of the file. A function declaration must contain a '(' after the name.");
            };

            if *tok.getKind() != TokenType::PuncOpenParen
            {
                panic!("An error has occurred at {}. A function declaration must contain a '(' after the name.", tok.getLoc());
            }

            let Some(tok) = self.iter.next() else
            {
                panic!("An error has occurred at the end of the file. A function must contain ')' after '('.");
            };

            if *tok.getKind() != TokenType::PuncCloseParen
            {
                panic!("An error has occurred at {}. A function must contain a ')' after the '('", tok.getLoc());
            }

            let Some(tok) = self.iter.next() else
            {
                panic!("An error has occurred at the end of the file. A function must contain a '{{' after it's header.");
            };

            if *tok.getKind() != TokenType::PuncOpenBrace
            {
                panic!("An error has occurred at {}. A function must contain a '{{' after it's header.", tok.getLoc());
            }

            let mut function = Function::new(funcType, funcName);

            while let Some(next) = self.iter.peek() && *next.getKind() != TokenType::PuncCloseBrace
            {

                function.addStatement(self.parseExpression());
            }

            let Some(tok) = self.iter.next() else
            {
                panic!("An error has occurred at the end of the file. A function declaration must end in a '}}'.");
            };

            if *tok.getKind() != TokenType::PuncCloseBrace
            {
                panic!("An error has occurred at {}. A function declaration must end in a '}}'.", tok.getLoc());
            }

            self.ast.addFunction(function);
        }
    }

    pub fn getAst(&self) -> &GlobalScope
    {
        &self.ast
    }

    fn parseExpression(&mut self) -> Node
    {
        self.parseVarAssign()
    }

    fn parseVarAssign(&mut self) -> Node
    {
        todo!()
    }
}