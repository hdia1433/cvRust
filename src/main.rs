#![allow(non_snake_case)]

use std::{env, fs, path::Path, process::Command};
use inkwell::{context::Context, targets::{Target, InitializationConfig, TargetMachine, RelocMode, CodeModel, FileType}, OptimizationLevel, passes::PassBuilderOptions};

pub mod model;

use model::Lexer;
use model::Parser;
use model::SemanticAnalyser;
use model::IRGenerator;
use model::Type;

fn main()
{
    let debug = if cfg!(debug_assertions)
    {
        true
    }
    else 
    {
        false
    };

    let source = fs::read_to_string(env::args().nth(1).expect("Expected file argument")).expect("Failed to read file");

    let mut lexer = Lexer::new(&source);
    lexer.tokenise();

    if debug
    {
        lexer.toFile().expect("Lexer failed to write to file.");
    }

    let mut parser = Parser::new(lexer.getTokens());
    parser.parse();

    if debug
    {
        parser.toFile().expect("Parser failed to write to file.");
    }

    let mut semanticAnalyser = SemanticAnalyser::new();

    semanticAnalyser.analyse(parser.getAstMut());

    let context = Context::create();
    let mut irGenerator = IRGenerator::new(&context);

    irGenerator.translate(parser.getAst());

    if debug
    {
        irGenerator.toFile();
    }

    Target::initialize_native(&InitializationConfig::default()).expect("Failed to initialise inkwell");
    let triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&triple).expect("Failed to get the target from the triple");

    let targetMachine = target.create_target_machine(&triple, "generic", "", OptimizationLevel::Aggressive, RelocMode::Default, CodeModel::Default).expect("Failed to create target machine");

    let module = irGenerator.getModule();

    module.run_passes("default<O3>", &targetMachine, PassBuilderOptions::create()).expect("Failed to optimise ir");

    if debug
    {
        module.print_to_file("optimisedIr.ll").expect("Failed to output optimised ir to file");
    }

    module.set_triple(&triple);
    module.set_data_layout(&targetMachine.get_target_data().get_data_layout());

    if debug
    {
        targetMachine.write_to_file(module, FileType::Assembly, Path::new("program.asm")).expect("Failed to write to assembly file.");
    }
    targetMachine.write_to_file(module, FileType::Object, Path::new("program.o")).expect("Failed to write to object file.");

    Command::new("clang")
        .args
        ([
            "program.o",
            "-o",
            "program"
        ])
        .status()
        .expect("Failed to compile object file");
}
