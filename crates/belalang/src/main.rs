#![allow(clippy::upper_case_acronyms)]

use belalang::execute_file;
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("belalang")
        .version("1.0")
        .about("Belalang CLI")
        .arg(
            Arg::new("filename")
                .help("Directly run a Belalang source code (.bel) (if no subcommand is given)")
                .index(1),
        )
        .subcommand(
            Command::new("build")
                .about("Build a Belalang project")
                .alias("b")
                .arg(Arg::new("directory").required(true).index(1)),
        )
        .subcommand(
            Command::new("run")
                .about("Run a Belalang project")
                .alias("r")
                .arg(Arg::new("directory").required(true).index(1)),
        )
        .subcommand(
            Command::new("compile")
                .about("Compile a Belalang source code (.bel) to Belalang byte code (.belc)")
                .alias("c")
                .arg(Arg::new("filename").required(true).index(1)),
        )
        .subcommand(
            Command::new("exec")
                .about("Execute a Belalang byte code (.belc)")
                .alias("x")
                .arg(Arg::new("filename").required(true).index(1)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("build", sub_m)) => {
            let directory = sub_m.get_one::<String>("directory").unwrap();
            println!("Building: {}", directory);
        }
        Some(("run", sub_m)) => {
            let directory = sub_m.get_one::<String>("directory").unwrap();
            println!("Running: {}", directory);
        }
        Some(("compile", sub_m)) => {
            let filename = sub_m.get_one::<String>("filename").unwrap();
            println!("Compiling: {}", filename);
        }
        Some(("exec", sub_m)) => {
            let filename = sub_m.get_one::<String>("filename").unwrap();
            println!("Executing: {}", filename);
        }
        Some(("completions", sub_m)) => {
            let shell = sub_m.get_one::<String>("shell").unwrap();
            println!("Generating completions for: {}", shell);
        }
        None => {
            if let Some(filename) = matches.get_one::<String>("filename") {
                execute_file(filename.into()).unwrap();
            } else {
                println!("Running REPL");
            }
        }
        _ => unreachable!(),
    }
}

