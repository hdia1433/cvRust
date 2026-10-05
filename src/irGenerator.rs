use crate::{nodes::{Function, GlobalScope, Node, VariableDeclaration, binaryOperation::{BinaryOperation, Op}}};
use inkwell::{builder::Builder, context::Context, module::Module, values::{FunctionValue, PointerValue}};
use std::collections::HashMap;

pub struct IRGenerator<'a>
{
    ast: &'a GlobalScope,
    context: &'a Context,
    module: Module<'a>,
    builder: Builder<'a>,
    funcs: HashMap<&'a str, FunctionValue<'a>>,
    vars: HashMap<&'a str, PointerValue<'a>>
}

impl<'a> IRGenerator<'a>
{
    pub fn new(ast: &'a GlobalScope, context: &'a Context) -> Self
    {
        let module = context.create_module("cv_program");
        let builder = context.create_builder();

        Self {ast, context, module, builder, funcs: HashMap::new(), vars: HashMap::new()}
    }

    pub fn getModule(&self) -> &Module<'a>
    {
        &self.module
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
        let function = self.module.add_function(
            if func.getName() == "main"
            {
                "__cv__main"
            }
            else 
            {
                func.getName()
            }, 
            funcType, None);

        let entry = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(entry);

        for node in func.getBody()
        {
            self.translateNode(node);
        }

        if *func.getNoReturn()
        {
            self.builder.build_return(None).expect("Failed to add a void return");
        }

        self.funcs.insert(func.getName(), function);
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

        let llvmMain = self.module.add_function("main", self.context.i32_type().fn_type(&[], false), None);

        let entry = self.context.append_basic_block(llvmMain, "entry");
        self.builder.position_at_end(entry);

        self.builder.build_call(*self.funcs.get("main").expect("Failed to get main function"), &[], "main").expect("Failed to add a call to cv's main to llvm's main function");
        self.builder.build_return(Some(&self.context.i32_type().const_int(0, false))).expect("Failed to add a return statement to llvm's main function");

        self.module.print_to_file("ir.ll").expect("Failed to write to ir file.");
    }
}