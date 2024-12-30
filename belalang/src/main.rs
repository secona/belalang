#![allow(clippy::upper_case_acronyms)]

use belalang::execute_file;
use clap::Parser;
use std::error::Error;
use std::path::PathBuf;

#[derive(clap::Parser)]
struct CLI {
    filename: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = CLI::parse();

    if let Err(err) = execute_file(cli.filename) {
        eprintln!("{}", err);
    }

    Ok(())
}
