use crate::model::{Type, nodes::{Function, GlobalScope, Node, VariableDeclaration, VariableAccess, binaryOperation::{BinaryOperation, Op}, Conversion}};
use inkwell::{builder::Builder, context::Context, module::Module, values::{BasicValueEnum, FunctionValue, PointerValue}};
use std::collections::HashMap;



pub struct IRGenerator<'a>
{
    context: &'a Context,
    module: Module<'a>,
    builder: Builder<'a>,
    funcs: HashMap<String, FunctionValue<'a>>,
    vars: HashMap<String, PointerValue<'a>>
}

impl<'a> IRGenerator<'a>
{
    pub fn new(context: &'a Context) -> Self
    {
        let module = context.create_module("cv_program");
        let builder = context.create_builder();

        Self {context, module, builder, funcs: HashMap::new(), vars: HashMap::new()}
    }

    pub fn getModule(&self) -> &Module<'a>
    {
        &self.module
    }

    fn translateGlobalScope(&mut self, globalScope: &GlobalScope)
    {
        for func in globalScope.getFunctions()
        {
            self.translateFunction(func);
        }
    }

    fn translateFunction(&mut self, func: &Function)
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

        self.funcs.insert(func.getName().to_string(), function);
    }

    pub fn translateVarDecl(&mut self, varDecl: &VariableDeclaration) -> Option<BasicValueEnum<'a>>
    {
        let varType = varDecl.getVarType().varType(&self.context);

        let var = self.builder.build_alloca(varType, varDecl.getName()).expect("Failed to allocate a variable");

        self.vars.insert(varDecl.getName().to_string(), var);

        None
    }

    pub fn translateVarAccess(&self, varAccess: &VariableAccess) -> BasicValueEnum<'a>
    {
        self.builder.build_load(varAccess.getVarType().varType(&self.context), *self.vars.get(varAccess.getName()).expect("Failed to retrieve variable"), format!("{}_value", varAccess.getName()).as_str()).expect("Failed to get value from variable")
    }

    fn translateNode(&mut self, node: &Node) -> Option<BasicValueEnum<'a>>
    {
        match node 
        {
            Node::VariableDeclaration(varDecl) => self.translateVarDecl(varDecl),
            Node::VariableAccess(varAccess) => Some(self.translateVarAccess(varAccess)),
            Node::BinaryOperation(binaryOp) => self.translateBinaryOp(binaryOp),
            Node::Conversion(conversion) => self.translateConversion(conversion),
            Node::Literal(literal) => Some(literal.getValue().intoBasicValue(&self.context)),
            Node::Error => panic!("Error node found")
        }
    }

    fn translateBinaryOp(&mut self, binaryOp: &BinaryOperation) -> Option<BasicValueEnum<'a>>
    {
        if *binaryOp.getOp() == Op::Assign
        {
            let var = match binaryOp.getLhs().as_ref()
            {
                Node::VariableDeclaration(varDecl) =>
                {
                    self.translateVarDecl(varDecl);

                    *self.vars.get(varDecl.getName()).expect("Couldn't find variable")
                },
                Node::VariableAccess(varAccess) =>
                {
                    *self.vars.get(varAccess.getName()).expect("Failed to find variable")
                },
                _ => panic!("Only a variable declaration can be assigned to.")
            };

            let Some(value) = self.translateNode(binaryOp.getRhs().as_ref()) else
            {
                panic!("A variable must be assigned to a basic value")
            };

            self.builder.build_store(var, value).expect("Failed to write to a variable");

            Some(value)
        }
        else 
        {
            let lhs = self.translateNode(binaryOp.getLhs()).expect("LHS Wasn't a basic value");
            let rhs = self.translateNode(binaryOp.getRhs()).expect("RHS wasn't a basic value");

            Some(match lhs
            {
                BasicValueEnum::IntValue(lhs) => 
                {
                    let BasicValueEnum::IntValue(rhs) = rhs else
                    {
                        panic!("RHS Wasn't an int value")
                    };

                    BasicValueEnum::IntValue(match binaryOp.getOp()
                    {
                        Op::Add => self.builder.build_int_add(lhs, rhs, "result"),
                        Op::Sub => self.builder.build_int_sub(lhs, rhs, "Result"),
                        Op::Mul => self.builder.build_int_mul(lhs, rhs, "result"),
                        Op::Div => self.builder.build_int_signed_div(lhs, rhs, "result"),
                        Op::Assign => unreachable!()
                    }.expect("Failed to build binary operation instruction"))
                },
                BasicValueEnum::FloatValue(lhs) =>
                {
                    let BasicValueEnum::FloatValue(rhs) = rhs else
                    {
                        panic!("RHS Wasn't a float value")
                    };

                    BasicValueEnum::FloatValue(match binaryOp.getOp()
                    {
                        Op::Add => self.builder.build_float_add(lhs, rhs, "result"),
                        Op::Sub => self.builder.build_float_sub(lhs, rhs, "result"),
                        Op::Mul => self.builder.build_float_mul(lhs, rhs, "result"),
                        Op::Div => self.builder.build_float_div(lhs, rhs, "result"),
                        Op::Assign => unreachable!()
                    }.expect("Failed to build binary op instruction"))
                },
                _ => unreachable!()
            })
        }
    }

    fn translateConversion(&mut self, conversion: &Conversion) -> Option<BasicValueEnum<'a>>
    {
        match (conversion.getInitialType(), conversion.getType())
        {
            (Type::Int, Type::Float) =>
            {
                let BasicValueEnum::IntValue(integer) = self.translateNode(conversion.getNode()).expect("node wasn't a basic value") else
                {
                    panic!("Node wasn't an integer value")
                };

                Some(BasicValueEnum::FloatValue(self.builder.build_signed_int_to_float(integer, self.context.f32_type(), "conversion").expect("Failed to build casting instruction")))
            },
            (Type::Float, Type::Int) =>
            {
                let BasicValueEnum::FloatValue(float) = self.translateNode(conversion.getNode()).expect("Node wasn't a basic value")else
                {
                    panic!("Node wasn't a float value");
                };

                Some(BasicValueEnum::IntValue(self.builder.build_float_to_signed_int(float, self.context.i32_type(), "conversion").expect("Failed to build casting")))
            },
            _ => panic!("Invalid conversion")
        }
    }
}

impl IRGenerator<'_>
{
    pub fn translate(&mut self, ast: &GlobalScope)
    {
        self.translateGlobalScope(ast);

        let llvmMain = self.module.add_function("main", self.context.i32_type().fn_type(&[], false), None);

        let entry = self.context.append_basic_block(llvmMain, "entry");
        self.builder.position_at_end(entry);

        self.builder.build_call(*self.funcs.get("main").expect("Failed to get main function"), &[], "main").expect("Failed to add a call to cv's main to llvm's main function");
        self.builder.build_return(Some(&self.context.i32_type().const_int(0, false))).expect("Failed to add a return statement to llvm's main function");
    }

    pub fn toFile(&self)
    {
        self.module.print_to_file("ir.ll").expect("Failed to write ir to file");
    }
}