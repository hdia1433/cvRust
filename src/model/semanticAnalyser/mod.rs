use crate::model::{Type, nodes::{Conversion, Function, GlobalScope, Node, VariableAccess, VariableDeclaration, binaryOperation::{BinaryOperation, Op}}};

mod semanticScope;

pub use semanticScope::SemanticScope;

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
        self.analyseNode(binaryOp.getLhsMut());
        self.analyseNode(binaryOp.getRhsMut());

        if binaryOp.getLhs().getType() != binaryOp.getRhs().getType()
        {
            match (binaryOp.getLhs().getType(), binaryOp.getOp(), binaryOp.getRhs().getType())
            {
                (Type::Int, op, Type::Float) if *op != Op::Assign => binaryOp.convertLhs(Type::Float),
                _ => binaryOp.convertRhs(binaryOp.getLhs().getType())
            }
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