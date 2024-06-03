use belalang::evaluator::Evaluator;
use belalang::lexer::Lexer;
use belalang::parser::Parser;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use rustyline::Result;

fn main() -> Result<()> {
    println!("Welcome to Belalang REPL v{}!\n", env!("CARGO_PKG_VERSION"));

    let mut rl = DefaultEditor::new()?;
    let mut ev = Evaluator::default();

    loop {
        match rl.readline(">> ") {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());

                let lexer = Lexer::new(line.as_bytes().into());
                let mut parser = Parser::new(lexer);

                match parser.parse_program() {
                    Ok(program) => match ev.evaluate_statements(program.statements) {
                        Ok(evaluated) => println!("{}", evaluated),
                        Err(msg) => println!("{}", msg),
                    },
                    Err(errors) => {
                        println!("parser errors:");
                        for error in errors {
                            println!("- {}", error);
                        }
                    }
                };
            }
            Err(ReadlineError::Interrupted) => {}
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
