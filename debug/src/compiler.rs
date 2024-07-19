use std::error::Error;

use belalang_comp::compiler::CompilerBuilder;
use belalang_core::{lexer::Lexer, parser::Parser};
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn compile(line: String) -> Result<(), Box<dyn Error>> {
    let lexer = Lexer::new(line.as_bytes());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program()?;

    let mut compiler = CompilerBuilder::default().build();
    let code = compiler.compile_program(program)?;

    println!("[instructions]");
    for (i, inst) in code.instructions.iter().enumerate() {
        println!("{:#06x}: {:#04x}", i, inst);
    }

    println!("\n[constants]");
    for (i, constant) in code.constants.iter().enumerate() {
        println!("{:#04x}: {:?}", i, constant);
    }

    println!("\n[symbols]");
    println!("{:#?}", compiler.scope.current().symbol_store);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut rl = DefaultEditor::new()?;

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                if let Err(err) = compile(line) {
                    println!("ERROR: {}", err);
                }
            }
            Err(ReadlineError::Interrupted) => (),
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("error reading line: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
