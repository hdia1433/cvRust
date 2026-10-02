use crate::{nodes::{Function, GlobalScope, Node, VariableDeclaration, binaryOperation::{BinaryOperation, Op}}};
use inkwell::{context::Context, module::Module, builder::Builder, values::PointerValue};
use std::collections::HashMap;

pub struct IRGenerator<'a>
{
    ast: &'a GlobalScope,
    context: &'a Context,
    module: Module<'a>,
    builder: Builder<'a>,
    vars: HashMap<&'a str, PointerValue<'a>>
}

impl<'a> IRGenerator<'a>
{
    pub fn new(ast: &'a GlobalScope, context: &'a Context) -> Self
    {
        let module = context.create_module("cv_program");
        let builder = context.create_builder();

        Self {ast, context, module, builder, vars: HashMap::new()}
    }

    fn translateGlobalScope(&mut self, globalScope: &'a GlobalScope)
    {
        for func in globalScope.getFunctions()
        {
            self.translateFunction(func);
        }
    }

    fn translateFunction(&mut self, func: &'a Function)
    {
        let funcType = func.getType().fnType(self.context,&[], false);
        let function = self.module.add_function(func.getName(), funcType, None);

        let entry = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(entry);

        for node in func.getBody()
        {
            self.translateNode(node);
        }
    }

    pub fn translateVarDecl(&mut self, varDecl: &'a VariableDeclaration)
    {
        let var = self.builder.build_alloca(varDecl.getVarType().varType(&self.context), varDecl.getName()).expect("Failed to allocate a variable");

        self.vars.insert(varDecl.getName(), var);
    }

    fn translateNode(&mut self, node: &'a Node)
    {
        match node 
        {
            Node::VariableDeclaration(varDecl) => self.translateVarDecl(varDecl),
            Node::BinaryOperation(binaryOp) => self.translateBinaryOp(binaryOp),
            _ => panic!("A literal cannot be translated")
        };
    }

    pub fn translateBinaryOp(&mut self, binaryOp: &'a BinaryOperation)
    {
        if *binaryOp.getOp() == Op::Assign
        {
            match binaryOp.getLhs().as_ref()
            {
                Node::VariableDeclaration(varDecl) =>
                {
                    self.translateVarDecl(varDecl);

                    let var = *self.vars.get(varDecl.getName()).expect("Couldn't find variable");

                    let value = match binaryOp.getRhs().as_ref()
                    {
                        Node::Literal(literal) => literal.getValue().intoBasicValue(&self.context),
                        _ => panic!("A variable needs to be a assigned to a literal")
                    };

                    self.builder.build_store(var, value).expect("Failed to write to a variable");
                },
                _ => panic!("Only a variable declaration can be assigned to.")
            }
        }
    }
}

impl IRGenerator<'_>
{
    pub fn translate(&mut self)
    {
        self.translateGlobalScope(self.ast);

        self.module.print_to_file("ir.txt").expect("Failed to write to ir file.");
    }
}