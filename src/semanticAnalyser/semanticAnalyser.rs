use std::{ops::Deref};

use crate::{errors::{CvError::{self, RuntimeError}, CvResult, RuntimeError::InvalidConversion}, parser::ast::{Type, nodes::{Conversion, Function, GlobalScope, Literal, Node, VariableAccess, VariableDeclaration, binaryOperation::{BinaryOperation, Op as BinOp}, literal::LiteralType, unaryOperation::{Op as UnOp, UnaryOperation}}}};

pub use super::SemanticScope;

pub struct SemanticAnalyser
{
    varScopes: Vec<SemanticScope>
}

impl SemanticAnalyser
{
    pub fn new() -> Self
    {
        Self {varScopes: Vec::new()}
    }

    pub fn analyse(&mut self, ast: &mut GlobalScope) -> CvResult<()>
    {
        let functions = ast.getFunctionsMut();

        for func in functions
        {
            self.analyseFunction(func)?;
        }

        Ok(())
    }

    fn analyseFunction(&mut self, func: &mut Function) -> CvResult<()>
    {
        self.varScopes.push(SemanticScope::new());

        for statement in func.getBodyMut()
        {
            self.analyseNode(statement)?;
        }

        Ok(())
    }

    fn analyseVarDecl(&mut self, varDecl: &mut VariableDeclaration)
    {
        if *varDecl.getVarType() == Type::Void
        {
            panic!("A variable cannot be of type 'void'");
        }

        self.varScopes.last_mut().expect("Failed to get last value in varScopes").addVar(varDecl.clone());
    }

    fn analyseVarAccess(&mut self, varAccess: &mut VariableAccess)
    {
        let exists = self.checkForVar(varAccess.getName());

        let Some(varType) = exists else
        {
            panic!("The variable is undeclared");
        };

        varAccess.setVarType(varType);
    }

    fn analyseBinaryOp(&mut self, binaryOp: &mut BinaryOperation) -> Result<(), CvError>
    {
        self.analyseNode(binaryOp.getLhsMut())?;
        self.analyseNode(binaryOp.getRhsMut())?;

        if &*binaryOp.getLhs().getType() != &*binaryOp.getRhs().getType()
        {
            let lhsType: Type = binaryOp.getLhs().getType().deref().clone();
            let op: BinOp = binaryOp.getOp().clone();
            let rhsType: Type = binaryOp.getRhs().getType().deref().clone();

            match (lhsType, op, rhsType)
            {
                (Type::Int, op, Type::Float) if op != BinOp::Assign => binaryOp.convertLhs(Type::Float),
                (type1, _, type2) if type1 != type2 && (type1 == Type::Char || type2 == Type::Char) => panic!("<char> cannot be implicitly converted into another type"),
                (Type::Array(varSize, varType), BinOp::Assign, Type::Array(arrSize, arrType)) if varSize == 0 && arrSize != 0 =>
                {
                    if let Node::VariableDeclaration(lhs) = binaryOp.getLhsMut()
                    {
                        let lhsType: &mut Type = lhs.getVarTypeMut();
                            
                        if let Type::Array(size, _) = lhsType
                        {
                            *size = arrSize;
                        }

                        for scope in self.varScopes.iter_mut().rev()
                        {
                            let vars: &mut Vec<VariableDeclaration> = scope.getVarsMut();

                            for i in (0..vars.len()).rev()
                            {
                                if vars[i].getName() == lhs.getName()
                                {
                                    vars[i] = lhs.clone();
                                }
                            }
                        }
                    }

                    if varType != arrType
                    {
                        binaryOp.convertRhs(binaryOp.getLhs().getType().deref().clone());
                    }
                },
                _ => binaryOp.convertRhs(binaryOp.getLhs().getType().deref().clone())
            }
        }

        if let Node::Conversion(conversion) = binaryOp.getRhsMut()
        {
            self.analyseConversion(conversion)?;
        }

        Ok(())
    }

    fn analyseUnaryOp(&mut self, unaryOp: &mut UnaryOperation) -> CvResult<()>
    {
        self.analyseNode(unaryOp.getNodeMut())?;

        match (unaryOp.getOp(), unaryOp.getNode().getType())
        {
            (UnOp::Dereference, nodeType) if !matches!(&*nodeType, Type::Pointer(_)) => panic!("Cannot dereference a node that isn't a pointer"),
            _ => ()
        }

        match (unaryOp.getOp(), unaryOp.getNode())
        {
            (UnOp::Dereference | UnOp::AddressOf, node) if !matches!(node, Node::VariableAccess(_)) && !matches!(node, Node::UnaryOperation(_)) => panic!("Can only dereference or get the memory address of variables"),
            _ => ()
        }

        Ok(())
    }

    fn analyseLiteral(&mut self, literal: &mut Literal) -> CvResult<()>
    {
        if let LiteralType::Array(arr) = literal.getValueMut()
        {
            let mut arrType: Option<Type> = None;

            for value in arr
            {
                self.analyseNode(value)?;

                match &arrType
                {
                    None => arrType = Some(value.getType().deref().clone()),
                    Some(arrType) => if *value.getType() != *arrType
                    {
                        *value = Node::Conversion(Conversion::new(value.clone(), arrType.clone()));
                        self.analyseNode(value)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn analyseConversion(&mut self, conversion: &mut Conversion) -> CvResult<()>
    {
        self.analyseNode(conversion.getNodeMut())?;

        conversion.getNode();

        let mut type1: &Type = &*conversion.getInitialType();
        let mut type2: &Type = conversion.getType();

        if let Type::Array(size1, inType1) = type1 && let Type::Array(size2, inType2) = type2 && size1 == size2
        {
            type1 = inType1;
            type2 = inType2;
        }

        match (type1, type2)
        {
            (Type::Int, type2) if *type2 == Type::Float || *type2 == Type::Bool => (),
            (Type::Float, type2) if *type2 == Type::Int || *type2 == Type::Bool => (),
            (Type::Bool, type2) if *type2 == Type::Float || *type2 == Type::Int => (),
            (Type::Pointer(_), Type::Pointer(_)) => (),
            _ => return Err(RuntimeError(InvalidConversion((*conversion.getInitialType()).clone(), (*conversion.getType()).clone())))
        }

        Ok(())
    }

    fn analyseNode(&mut self, node: &mut Node) -> Result<(), CvError>
    {
        match node
        {
            Node::VariableDeclaration(varDecl) => self.analyseVarDecl(varDecl),
            Node::VariableAccess(varAccess) => self.analyseVarAccess(varAccess),
            Node::BinaryOperation(binaryOp) => self.analyseBinaryOp(binaryOp)?,
            Node::Conversion(conversion) => self.analyseConversion(conversion)?,
            Node::Literal(literal) => self.analyseLiteral(literal)?,
            Node::UnaryOperation(unaryOp) => self.analyseUnaryOp(unaryOp)?,
            Node::Error => panic!("Error node found")
        }

        Ok(())
    }

    fn checkForVar(&mut self, name: &str) -> Option<Type>
    {
        for scope in self.varScopes.iter().rev()
        {
            for var in scope.getVars().iter().rev()
            {
                if var.getName() == name
                {
                    return Some(var.getVarType().clone());
                }
            }
        }

        None
    }
}