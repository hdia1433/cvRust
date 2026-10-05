use super::{token::{Token, TokenType}};
use crate::nodes::{GlobalScope, Node, Function, VariableDeclaration, BinaryOperation, binaryOperation::Op, Literal, literal::LiteralType};
use std::{iter::Peekable, slice::Iter, io::{Result, Write}, fs::File};


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
}

impl Parser<'_>
{
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

            let function = self.parseFunction(funcType, funcName);

            self.ast.addFunction(function);
        }
    }

    pub fn getAst(&self) -> &GlobalScope
    {
        &self.ast
    }

    pub fn toFile(&self) -> Result<()>
    {
        let mut file = File::create("ast.txt")?;

        write!(file, "{}", self.ast)?;

        Ok(())
    }

    fn parseFunction(&mut self, funcType: TokenType, funcName: &str) -> Function
    {
        let mut function = Function::new(funcType, funcName);

        while let Some(next) = self.iter.peek() && *next.getKind() != TokenType::PuncCloseBrace
        {
            function.addStatement(self.parseStatement());
        }

        let Some(tok) = self.iter.next() else
        {
            panic!("An error has occurred at the end of the file. A function declaration must end in a '}}'.");
        };

        if *tok.getKind() != TokenType::PuncCloseBrace
        {
            panic!("An error has occurred at {}. A function declaration must end in a '}}'.", tok.getLoc());
        }

        function.setRoReturn(true);

        function
    }

    fn parseStatement(&mut self) -> Node
    {
        let statement = self.parseExpression();

        let Some(tok) = self.iter.next() else
        {
            panic!("An error has occurred at the end of the file. A semi-colon is needed to end a statement.");
        };

        if *tok.getKind() != TokenType::PuncSemi
        {
            panic!("An error has occurred at {}. A semi-colon is needed to end a statement.", tok.getLoc());
        }

        statement
    }

    fn parseExpression(&mut self) -> Node
    {
        self.parseVarAssign()
    }

    fn parseVarAssign(&mut self) -> Node
    {
        let mut lhs = self.parsePrimary();

        if let Some(next) = self.iter.peek() && *next.getKind() == TokenType::OpAssign
        {
            self.iter.next().expect("Failed to get '=' token.");

            let rhs = self.parsePrimary();

            lhs = Node::BinaryOperation(BinaryOperation::new(lhs, Op::Assign, rhs))
        }

        lhs
    }

    fn parsePrimary(&mut self) -> Node
    {
        let Some(tok) = self.iter.next() else 
        {
            panic!("An error has occurred at the end of the file. A primary was expected.");
        };

        if tok.getKind().isType()
        {
            let varType = tok.getKind().clone();

            let Some(tok) = self.iter.next() else
            {
                panic!("An error has occurred at the end of the file. A variable name was expected.");
            };

            let TokenType::Identifier(varName) = tok.getKind() else
            {
                panic!("An error has occurred at {}. A variable name was expected.", tok.getLoc());
            };

            return Node::VariableDeclaration(self.parseVariableDeclaration(varType, varName));
        }

        match tok.getKind()
        {
            TokenType::LitInteger(integer) => 
            {
                Node::Literal(Literal::new(LiteralType::Integer(*integer as i32)))
            },
            _ => panic!("Invalid primary")
        }
    }

    fn parseVariableDeclaration(&self, varType: TokenType, varName: &str) -> VariableDeclaration
    {
        VariableDeclaration::new(varType, varName)
    }
}