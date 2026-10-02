#![allow(non_snake_case)]

use std::{env, fs};
use inkwell::context::Context;

mod lexer;
mod parser;
mod semanticAnalyser;
mod irGenerator;
pub mod token;
pub mod nodes;
pub mod types;

pub use lexer::Lexer;
pub use parser::Parser;
pub use semanticAnalyser::SemanticAnalyser;
pub use irGenerator::IRGenerator;
pub use token::Token;
pub use types::Type;

fn main()
{
    let source = fs::read_to_string(env::args().nth(1).expect("Expected file argument")).expect("Failed to read file");

    let mut lexer = Lexer::new(&source);
    lexer.tokenise();

    lexer.toFile().expect("Lexer failed to write to file.");

    let mut parser = Parser::new(lexer.getTokens());
    parser.parse();

    parser.toFile().expect("Parser failed to write to file.");

    let mut semanticAnalyser = SemanticAnalyser::new(parser.getAst());

    semanticAnalyser.analyse();

    let context = Context::create();
    let mut irGenerator = IRGenerator::new(parser.getAst(), &context);

    irGenerator.translate();
}
