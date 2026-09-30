use crate::parser::node::{GlobalScope, Function, VariableDeclaration, BinaryOperation, Node};

mod semanticScope;

pub use semanticScope::SemanticScope;

pub struct SemanticAnalyser<'a>
{
    ast: &'a GlobalScope,
    varScopes: Vec<SemanticScope<'a>>
}

impl<'a> SemanticAnalyser<'a>
{
    pub fn new(ast: &'a GlobalScope) -> Self
    {
        Self {ast, varScopes: Vec::new()}
    }

    pub fn analyse(&mut self)
    {
        self.analyseGlobalScope(self.ast);
    }

    fn analyseGlobalScope(&mut self, globalScope: &'a GlobalScope)
    {
        for func in globalScope.getFunctions()
        {
            self.analyseFunction(func);
        }
    }

    fn analyseFunction(&mut self, func: &'a Function)
    {
        self.varScopes.push(SemanticScope::new());

        for statement in func.getBody()
        {
            self.analyseNode(statement);
        }
    }

    fn analyseVarDecl(&mut self, varDecl: &'a VariableDeclaration)
    {
        self.varScopes.last_mut().expect("Failed to get last value in varScopes").addVar(varDecl);
    }

    fn analyseBinaryOp(&mut self, binaryOp: &'a BinaryOperation)
    {
        self.analyseNode(binaryOp.getLhs());
        self.analyseNode(binaryOp.getRhs());

        if binaryOp.getLhs().getType() != binaryOp.getRhs().getType()
        {
            panic!("Binary Operation type mismatch");
        }
    }

    fn analyseNode(&mut self, node: &'a Node)
    {
        match node
        {
            Node::VariableDeclaration(varDecl) => self.analyseVarDecl(varDecl),
            Node::BinaryOperation(binaryOp) => self.analyseBinaryOp(binaryOp),
            Node::Literal(_) => (),
        }
    }
}