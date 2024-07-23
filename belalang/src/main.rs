use belalang::{repl, run_file};
use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

#[derive(clap::Parser)]
struct CLI {
    filename: Option<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = CLI::parse();

    let result = match cli.filename {
        Some(filename) => run_file(filename),
        None => repl(),
    };

    if let Err(err) = result {
        eprintln!("{}", err);
    }

    Ok(())
}
