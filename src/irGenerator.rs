use crate::{errors::CompilerError::{ErrorNodeFound, ExpectedBasicValue, ExpectedNode, ExpectedPointerValue}, parser::ast::{Type, nodes::{Conversion, Function, GlobalScope, Node, VariableAccess, VariableDeclaration, binaryOperation::{BinaryOperation, Op as BinOp}, unaryOperation::{Op as UnOp, UnaryOperation}}}};
use inkwell::{FloatPredicate, IntPredicate, builder::Builder, context::Context, module::Module, values::{BasicValue, BasicValueEnum, FunctionValue, IntValue, PointerValue}};
use std::collections::HashMap;
use anyhow::{Ok, Result as AResult};



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

    fn translateGlobalScope(&mut self, globalScope: &GlobalScope) -> AResult<()>
    {
        for func in globalScope.getFunctions()
        {
            self.translateFunction(func)?;
        }

        Ok(())
    }

    fn translateFunction(&mut self, func: &Function) -> AResult<()>
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
            self.translateNode(node)?;
        }

        if *func.getNoReturn()
        {
            self.builder.build_return(None).expect("Failed to add a void return");
        }

        self.funcs.insert(func.getName().to_string(), function);

        Ok(())
    }

    pub fn translateVarDecl(&mut self, varDecl: &VariableDeclaration) -> AResult<PointerValue<'a>>
    {
        let varType = varDecl.getVarType().varType(&self.context);

        let var = self.builder.build_alloca(varType, varDecl.getName()).expect("Failed to allocate a variable");

        self.vars.insert(varDecl.getName().to_string(), var);
        self.builder.build_store(var, varType.const_zero())?;

        Ok(var)
    }

    pub fn translateVarAccess(&self, varAccess: &VariableAccess) -> AResult<PointerValue<'a>>
    {
        Ok(*self.vars.get(varAccess.getName()).ok_or(ExpectedPointerValue)?)
    }

    fn translateNode(&mut self, node: &Node) -> AResult<Option<BasicValueEnum<'a>>>
    {
        match node 
        {
            Node::VariableDeclaration(varDecl) => Ok(Some(self.translateVarDecl(varDecl)?.as_basic_value_enum())),
            Node::VariableAccess(varAccess) => Ok(Some(self.translateVarAccess(varAccess)?.as_basic_value_enum())),
            Node::BinaryOperation(binaryOp) => self.translateBinaryOp(binaryOp),
            Node::Conversion(conversion) => Ok(Some(self.translateConversion(conversion)?)),
            Node::Literal(literal) => Ok(Some(literal.getValue().intoBasicValue(&self.context, &self.builder)?)),
            Node::UnaryOperation(unaryOp) => Ok(Some(self.translateUnaryOp(unaryOp)?)),
            Node::Error => panic!("Error node found")
        }
    }

    fn translateNodeToValue(&mut self, node: &Node) -> AResult<BasicValueEnum<'a>>
    {
        match node
        {
            Node::VariableDeclaration(varDecl) =>
            {
                let ptr:PointerValue = self.translateVarDecl(varDecl)?;

                Ok(self.builder.build_load(varDecl.getVarType().varType(&self.context), ptr, &format!("{}_access", varDecl.getName()))?)
            },
            Node::VariableAccess(varAccess) =>
            {
                let ptr:PointerValue = self.translateVarAccess(varAccess)?;

                Ok(self.builder.build_load(varAccess.getVarType().varType(self.context), ptr, &format!("{}_access", varAccess.getName()))?)
            },
            Node::BinaryOperation(binaryOp) =>
            {
                let mut value:BasicValueEnum = self.translateBinaryOp(binaryOp)?.ok_or(ExpectedBasicValue)?;

                if let BasicValueEnum::PointerValue(ptr) = value
                {
                    value = self.builder.build_load(binaryOp.getType().varType(self.context), ptr, "result")?;
                }

                Ok(value)
            },
            Node::Conversion(conversion) => self.translateConversion(conversion),
            Node::Literal(literal) => Ok(literal.getValue().intoBasicValue(self.context, &self.builder)?),
            Node::UnaryOperation(unaryOp) =>
            {
                let mut value:BasicValueEnum = self.translateUnaryOp(unaryOp)?;

                if let BasicValueEnum::PointerValue(ptr) = value
                {
                    value = self.builder.build_load(unaryOp.getType().varType(self.context), ptr, "result")?;
                }

                Ok(value)
            },
            Node::Error => Err(ErrorNodeFound.into())
        }
    }

    fn translateBinaryOp(&mut self, binaryOp: &BinaryOperation) -> AResult<Option<BasicValueEnum<'a>>>
    {
        if *binaryOp.getOp() == BinOp::Assign
        {
            let var = match binaryOp.getLhs()
            {
                Node::VariableDeclaration(varDecl) =>
                {
                    self.translateVarDecl(varDecl)?;

                    *self.vars.get(varDecl.getName()).expect("Couldn't find variable")
                },
                Node::VariableAccess(varAccess) =>
                {
                    *self.vars.get(varAccess.getName()).expect("Failed to find variable")
                },
                Node::UnaryOperation(unaryOp) =>
                {
                    let BasicValueEnum::PointerValue(ptr) = self.translateUnaryOp(unaryOp)? else
                    {
                        return Err(ExpectedPointerValue.into())
                    };

                    ptr
                },
                _ => panic!("Only a variable declaration can be assigned to.")
            };

            let value = self.translateNodeToValue(binaryOp.getRhs())?;

            self.builder.build_store(var, value).expect("Failed to write to a variable");

            Ok(Some(value))
        }
        else 
        {
            let lhs: BasicValueEnum = self.translateNodeToValue(binaryOp.getLhs())?;
            let rhs:BasicValueEnum = self.translateNodeToValue(binaryOp.getRhs())?;

            Ok(Some(match lhs
            {
                BasicValueEnum::IntValue(lhs) => 
                {
                    let BasicValueEnum::IntValue(rhs) = rhs else
                    {
                        panic!("RHS Wasn't an int value")
                    };

                    BasicValueEnum::IntValue(match binaryOp.getOp()
                    {
                        BinOp::Add => self.builder.build_int_add(lhs, rhs, "result"),
                        BinOp::Sub => self.builder.build_int_sub(lhs, rhs, "Result"),
                        BinOp::Mul => self.builder.build_int_mul(lhs, rhs, "result"),
                        BinOp::Div => self.builder.build_int_signed_div(lhs, rhs, "result"),
                        BinOp::Equals => self.builder.build_int_compare(IntPredicate::EQ, lhs, rhs, "result"),
                        BinOp::Assign => unreachable!(),
                    }.expect("Failed to build binary operation instruction"))
                },
                BasicValueEnum::FloatValue(lhs) =>
                {
                    let BasicValueEnum::FloatValue(rhs) = rhs else
                    {
                        panic!("RHS Wasn't a float value")
                    };

                    match binaryOp.getOp()
                    {
                        BinOp::Add => BasicValueEnum::FloatValue(self.builder.build_float_add(lhs, rhs, "result").expect("Failed to build add operation")),
                        BinOp::Sub => BasicValueEnum::FloatValue(self.builder.build_float_sub(lhs, rhs, "result").expect("Failed to build sub operation")),
                        BinOp::Mul => BasicValueEnum::FloatValue(self.builder.build_float_mul(lhs, rhs, "result").expect("Failed to build mul operation")),
                        BinOp::Div => BasicValueEnum::FloatValue(self.builder.build_float_div(lhs, rhs, "result").expect("Failed to build dib operation")),
                        BinOp::Equals => BasicValueEnum::IntValue(self.builder.build_float_compare(FloatPredicate::OEQ, lhs, rhs, "result").expect("Failed to build compare operation")),
                        BinOp::Assign => unreachable!()
                    }
                },
                _ => unreachable!()
            }))
        }
    }

    fn translateUnaryOp(&mut self, unaryOp: &UnaryOperation) -> AResult<BasicValueEnum<'a>>
    {
        match unaryOp.getOp()
        {
            UnOp::AddressOf => 
            {
                let Node::VariableAccess(varAccess) = unaryOp.getNode() else
                {
                    panic!("Cannot get the address of a node that isn't a variable access")
                };

                Ok(BasicValueEnum::PointerValue(*self.vars.get(varAccess.getName()).expect("Failed to get variable")))
            },
            UnOp::Dereference => 
            {
                let value: BasicValueEnum = self.translateNode(unaryOp.getNode())?.expect("Failed to get value from node");

                let BasicValueEnum::PointerValue(ptr) = value else
                {
                    panic!("Can only dereference pointer values")
                };

                let Type::Pointer(subType) = &*unaryOp.getNode().getType() else
                {
                    panic!("Can only deference a value of pointer type");
                };

                Ok(self.builder.build_load(subType.varType(&self.context), ptr, "deref").expect("Failed to build load instruction"))
            },
            UnOp::Subscript(i) => 
            {
                let Node::VariableAccess(varAccess) = unaryOp.getNode() else
                {
                    return Err(ExpectedNode("Variable Access".into()).into());
                };
                let ptr: PointerValue = *self.vars.get(varAccess.getName()).ok_or(ExpectedPointerValue)?;
                let zero: IntValue = self.context.i32_type().const_int(0, false);
                let index: IntValue = self.translateNodeToValue(i)?.into_int_value();
                let elementPtr: PointerValue = unsafe
                {
                    self.builder.build_gep(varAccess.getType().varType(&self.context), ptr, &[zero, index], "ElementPtr")?
                };

                Ok(elementPtr.as_basic_value_enum())
            }
        }
    }

    fn translateConversion(&mut self, conversion: &Conversion) -> AResult<BasicValueEnum<'a>>
    {
        match (&*conversion.getInitialType(), conversion.getType())
        {
            (Type::Int, Type::Float) =>
            {
                let BasicValueEnum::IntValue(integer) = self.translateNodeToValue(conversion.getNode())? else
                {
                    panic!("Node wasn't an integer value")
                };

                Ok(BasicValueEnum::FloatValue(self.builder.build_signed_int_to_float(integer, self.context.f32_type(), "conversion").expect("Failed to build casting instruction")))
            },
            (Type::Float, Type::Int) =>
            {
                let BasicValueEnum::FloatValue(float) = self.translateNodeToValue(conversion.getNode())? else
                {
                    panic!("Node wasn't a float value");
                };

                Ok(BasicValueEnum::IntValue(self.builder.build_float_to_signed_int(float, self.context.i32_type(), "conversion").expect("Failed to build casting")))
            },
            _ => panic!("Invalid conversion")
        }
    }
}

impl IRGenerator<'_>
{
    pub fn translate(&mut self, ast: &GlobalScope) -> AResult<()>
    {
        self.translateGlobalScope(ast)?;

        let llvmMain = self.module.add_function("main", self.context.i32_type().fn_type(&[], false), None);

        let entry = self.context.append_basic_block(llvmMain, "entry");
        self.builder.position_at_end(entry);

        self.builder.build_call(*self.funcs.get("main").expect("Failed to get main function"), &[], "main").expect("Failed to add a call to cv's main to llvm's main function");
        self.builder.build_return(Some(&self.context.i32_type().const_int(0, false))).expect("Failed to add a return statement to llvm's main function");

        Ok(())
    }

    pub fn toFile(&self)
    {
        self.module.print_to_file("ir.ll").expect("Failed to write ir to file");
    }
}