//! The Vyro language toolchain as a library: lexer, parser, compiler, VM.

pub mod ast;
pub mod compiler;
pub mod lexer;
pub mod opcode;
pub mod parser;
pub mod server;
pub mod token;
pub mod value;
pub mod vm;

use compiler::Compiler;
use lexer::Lexer;
use parser::Parser;
use value::Function;
use vm::Vm;

/// Compile source text into a `main` Function, or return a diagnostic.
pub fn compile_source(src: &str) -> Result<Function, String> {
    let tokens = Lexer::new(src).tokenize()?;
    let program = Parser::new(tokens).parse()?;
    Compiler::compile(&program)
}

/// Compile and run source; returns everything printed.
pub fn run_source(src: &str) -> Result<String, String> {
    let main = compile_source(src)?;
    Vm::new().run(main)
}
