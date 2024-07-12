use std::{error::Error, path::PathBuf};

use belalang_comp::compiler::Compiler;
use belalang_core::{lexer::Lexer, parser::Parser};
use belalang_vm::vm::VM;
use rustyline::{error::ReadlineError, DefaultEditor};

pub fn run_file(_filename: PathBuf) -> Result<(), Box<dyn Error>> {
    todo!()
}

pub fn repl() -> Result<(), Box<dyn Error>> {
    println!("Welcome to Belalang REPL v{}!\n", env!("CARGO_PKG_VERSION"));

    let mut rl = DefaultEditor::new()?;
    let mut compiler = Compiler::default();
    let mut vm = VM::default();

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());

                let lexer = Lexer::new(line.as_bytes().into());
                let mut parser = Parser::new(lexer);

                match parser.parse_program() {
                    Ok(program) => match compiler.compile_program(program) {
                        Ok(mut code) => {
                            vm.append_code(&mut code);
                            match vm.run() {
                                Ok(_) => println!("{}", vm.last_popped),
                                Err(err) => println!("runtime error: {err}"),
                            }
                        },
                        Err(err) => println!("compile error: {err}"),
                    },
                    Err(err) => println!("parsing error: {err}"),
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
