use crate::parser::ast::{Type, nodes::{Function, GlobalScope, Node, VariableAccess, VariableDeclaration, binaryOperation::{BinaryOperation, Op as BinOp}, unaryOperation::{UnaryOperation, Op as UnOp}}};

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

    pub fn analyse(&mut self, ast: &mut GlobalScope)
    {
        let functions = ast.getFunctionsMut();

        for func in functions
        {
            self.analyseFunction(func);
        }
    }

    fn analyseFunction(&mut self, func: &mut Function)
    {
        self.varScopes.push(SemanticScope::new());

        for statement in func.getBodyMut()
        {
            self.analyseNode(statement);
        }
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

    fn analyseBinaryOp(&mut self, binaryOp: &mut BinaryOperation)
    {
        self.analyseNode(binaryOp.getRhsMut());
        self.analyseNode(binaryOp.getLhsMut());

        if binaryOp.getLhs().getType() != binaryOp.getRhs().getType()
        {
            match (binaryOp.getLhs().getType(), binaryOp.getOp(), binaryOp.getRhs().getType())
            {
                (Type::Int, op, Type::Float) if *op != BinOp::Assign => binaryOp.convertLhs(Type::Float),
                (type1, _, type2) if type1 == Type::Char || type2 == Type::Char => panic!("<char> cannot be implicitly converted into another type"),
                _ => binaryOp.convertRhs(binaryOp.getLhs().getType())
            }
        }
    }

    fn analyseUnaryOp(&mut self, unaryOp: &mut UnaryOperation)
    {
        self.analyseNode(unaryOp.getNodeMut());

        match (unaryOp.getOp(), unaryOp.getNode().getType())
        {
            (UnOp::Dereference, nodeType) if !matches!(nodeType, Type::Pointer(_)) => panic!("Cannot dereference a node that isn't a pointer"),
            _ => ()
        }

        match (unaryOp.getOp(), unaryOp.getNode())
        {
            (UnOp::Dereference | UnOp::AddressOf, node) if !matches!(node, Node::VariableAccess(_)) && !matches!(node, Node::UnaryOperation(_)) => panic!("Can only dereference or get the memory address of variables"),
            _ => ()
        }
    }

    fn analyseNode(&mut self, node: &mut Node)
    {
        match node
        {
            Node::VariableDeclaration(varDecl) => self.analyseVarDecl(varDecl),
            Node::VariableAccess(varAccess) => self.analyseVarAccess(varAccess),
            Node::BinaryOperation(binaryOp) => self.analyseBinaryOp(binaryOp),
            Node::Conversion(_) => (),
            Node::Literal(_) => (),
            Node::UnaryOperation(unaryOp) => self.analyseUnaryOp(unaryOp),
            Node::Error => panic!("Error node found")
        }
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