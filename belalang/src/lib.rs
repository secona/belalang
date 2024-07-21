use std::error::Error;
use std::path::PathBuf;

use belalang_comp::compiler::CompilerBuilder;
use belalang_core::lexer::Lexer;
use belalang_core::parser::Parser;
use belalang_vm::vm::VMBuilder;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

pub fn run_file(_filename: PathBuf) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn repl() -> Result<(), Box<dyn Error>> {
    println!("Welcome to Belalang REPL v{}!\n", env!("CARGO_PKG_VERSION"));

    let mut rl = DefaultEditor::new()?;
    let mut compiler = CompilerBuilder::default().incremental(true).build();
    let mut vm = VMBuilder::default().build();

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());

                let lexer = Lexer::new(line.as_bytes().into());
                let mut parser = Parser::new(lexer);

                let program = match parser.parse_program() {
                    Ok(program) => program,
                    Err(err) => {
                        println!("Parsing Error: {err}");
                        continue;
                    }
                };

                let code = match compiler.compile_program(program) {
                    Ok(code) => code,
                    Err(err) => {
                        println!("Compile Error: {err}");
                        continue;
                    }
                };

                match vm.run(code) {
                    Ok(_) => println!("{}", vm.last_popped),
                    Err(err) => println!("runtime error: {err}"),
                }
            }
            Err(ReadlineError::Interrupted) => {
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("\nSee you, space cowboy...");
                break;
            }
            Err(err) => {
                println!("Err: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
