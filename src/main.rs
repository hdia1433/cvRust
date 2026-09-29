#![allow(non_snake_case)]

use std::{env, fs};
use lexer::Lexer;

mod lexer;

fn main()
{
    let source = fs::read_to_string(env::args().nth(1).expect("Expected file argument")).expect("Failed to read file");

    let mut lexer = Lexer::new(&source);

    lexer.tokenise();
}
