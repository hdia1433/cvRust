#![allow(non_snake_case)]

use std::{env, fs};
use lexer::Lexer;
use parser::Parser;

mod lexer;
mod parser;

fn main()
{
    let source = fs::read_to_string(env::args().nth(1).expect("Expected file argument")).expect("Failed to read file");

    let mut lexer = Lexer::new(&source);
    lexer.tokenise();

    lexer.toFile().expect("Lexer failed to write to file.");

    let mut parser = Parser::new(lexer.getTokens());
    parser.parse();

    parser.toFile().expect("Parser failed to write to file.");
}
