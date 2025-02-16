use std::error::Error;
use std::io::{self, Write};

use belalang_compiler::ast::Parser;
use belalang_compiler::codegen::disassembler::disassemble;
use belalang_compiler::codegen::Compiler;
use belalang_compiler::tokens::Lexer;

fn compile(line: String) -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::new(line.as_bytes());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;

    let mut compiler = Compiler::default();
    let code = compiler.compile_program(program)?;

    let disassembled = disassemble(code.instructions);

    println!("===== BYTECODE =====");
    print!("{disassembled}");

    println!("\n===== CONSTANTS =====");
    for (i, constant) in code.constants.iter().enumerate() {
        println!("{:#04x}: {:?}", i, constant);
    }

    println!("\n===== SYMBOLS =====");
    println!("{:#?}", compiler.scope.current().symbol_store);

    println!();

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().is_empty() {
            println!();
            continue;
        }

        if let Err(error) = compile(input.clone()) {
            println!("ERROR: {}", error);
        }
    }
}
