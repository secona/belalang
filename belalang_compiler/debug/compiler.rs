use std::error::Error;
use std::io::{self, Write};

use belalang_compiler::compiler::CompilerBuilder;
use belalang_compiler::disassembly::disassemble;
use belalang_compiler::parser::Parser;
use belalang_compiler::lexer::Lexer;

fn compile(line: String) -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::new(line.as_bytes());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;

    let mut compiler = CompilerBuilder::default().build();
    let code = compiler.compile_program(program)?;

    let disassembled = disassemble(code.instructions);
    print!("{disassembled}");

    println!("\n[constants]");
    for (i, constant) in code.constants.iter().enumerate() {
        println!("{:#04x}: {:?}", i, constant);
    }

    println!("\n[symbols]");
    println!("{:#?}", compiler.scope.current().symbol_store);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        input.clear();
        io::stdin().read_line(&mut input).unwrap();

        if let Err(error) = compile(input.clone()) {
            println!("ERROR: {}", error);
        }
    }
}
