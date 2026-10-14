use crate::lexer::token::{Token, TokenType};
use crate::parser::ast::Type;
use crate::parser::ast::nodes::{BinaryOperation, Function, GlobalScope, Literal, Node, ToFile, VariableAccess, VariableDeclaration, binaryOperation::Op as OpBin, unaryOperation::{UnaryOperation, Op as OpUn}, literal::LiteralType};
use std::{iter::Peekable, slice::Iter, io::{Result, Write}, fs::File};
use bitvec::prelude::BitVec;


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

    pub fn getAstMut(&mut self) -> &mut GlobalScope
    {
        &mut self.ast
    }

    pub fn toFile(&self, path: &str) -> Result<()>
    {
        let mut file = File::create(path)?;

        write!(file, "ast")?;

        self.ast.toFile(&mut file, &mut BitVec::new(), true)?;

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

        function.setNoReturn(true);

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
        let mut lhs = self.parseEquality();

        if let Some(next) = self.iter.peek() && *next.getKind() == TokenType::OpAssign
        {
            self.iter.next().expect("Failed to get '=' token");

            let rhs = self.parseExpression();

            lhs = Node::BinaryOperation(BinaryOperation::new(lhs, OpBin::Assign, rhs))
        }

        lhs
    }

    fn parseEquality(&mut self) -> Node
    {
        let mut lhs = self.parseAdd();

        if let Some(next) = self.iter.peek() && *next.getKind() == TokenType::OpEquals
        {
            self.iter.next().expect("Failed to get '==' token");

            let rhs = self.parseExpression();
            lhs = Node::BinaryOperation(BinaryOperation::new(lhs, OpBin::Equals, rhs));
        }

        lhs
    }

    fn parseAdd(&mut self) -> Node
    {
        let mut lhs = self.parseMul();

        if let Some(next) = self.iter.peek() && (*next.getKind() == TokenType::OpPlus || *next.getKind() == TokenType::OpMinus)
        {
            let tok = self.iter.next().expect("Failed to get '+' or '-' token");

            let rhs = self.parseExpression();

            let op = match tok.getKind()
            {
                TokenType::OpPlus => OpBin::Add,
                TokenType::OpMinus => OpBin::Sub,
                _ => unreachable!()
            };

            lhs = Node::BinaryOperation(BinaryOperation::new(lhs, op, rhs));
        }

        lhs
    }

    fn parseMul(&mut self) -> Node
    {
        let mut lhs = self.parsePrefix();

        if let Some(next) = self.iter.peek() && (*next.getKind() == TokenType::OpStar || *next.getKind() == TokenType::OpSlash)
        {
            let tok: &Token = self.iter.next().expect("Failed to get '*' or '/' token");

            let rhs: Node = self.parseExpression();

            let op: OpBin = match tok.getKind()
            {
                TokenType::OpStar => OpBin::Mul,
                TokenType::OpSlash => OpBin::Div,
                _ => unreachable!()
            };

            lhs = Node::BinaryOperation(BinaryOperation::new(lhs, op, rhs));
        }

        lhs
    }

    fn parsePrefix(&mut self) -> Node
    {
        if let Some(next) = self.iter.peek() && (*next.getKind() == TokenType::OpReference || *next.getKind() == TokenType::OpStar)
        {
            let tok: &Token = self.iter.next().expect("Failed to get '&' token");

            let node:Node = self.parsePrimary();

            let op: OpUn = match *tok.getKind()
            {
                TokenType::OpReference => OpUn::AddressOf,
                TokenType::OpStar => OpUn::Dereference,
                _ => unreachable!()
            };

            Node::UnaryOperation(UnaryOperation::newPrefix(op, node))
        }
        else
        {
            self.parsePrimary()
        }
    }

    fn parsePrimary(&mut self) -> Node
    {
        let Some(tok) = self.iter.next() else 
        {
            panic!("An error has occurred at the end of the file. A primary was expected.");
        };

        if tok.getKind().isType()
        {
            let mut varType = Type::fromToken(tok.getKind().clone());

            while let Some(tok) = self.iter.next()
            {
                match tok.getKind()
                {
                    TokenType::Identifier(varName) => return Node::VariableDeclaration(self.parseVariableDeclaration(varType, varName)),
                    TokenType::OpStar => varType = Type::Pointer(Box::new(varType)),
                    _ => break
                }
            }

            panic!("Expected a variable name");
        }

        match tok.getKind()
        {
            TokenType::LitInteger(integer) => 
            {
                Node::Literal(Literal::new(LiteralType::Integer(*integer)))
            },
            TokenType::LitFloat(float) => Node::Literal(Literal::new(LiteralType::Float(*float))),
            TokenType::LitBool(boolean) => Node::Literal(Literal::new(LiteralType::Bool(*boolean))),
            TokenType::LitChar(character) => Node::Literal(Literal::new(LiteralType::Char(*character))),
            TokenType::Identifier(ident) => Node::VariableAccess(VariableAccess::new(ident)),
            _ => unreachable!()
        }
    }

    fn parseVariableDeclaration(&self, varType: Type, varName: &str) -> VariableDeclaration
    {
        VariableDeclaration::new(varType, varName)
    }
}