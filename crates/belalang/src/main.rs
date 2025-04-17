use belalang::{repl, run};
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
        .get_matches();

    match matches.subcommand() {
        None => match matches.get_one::<String>("filename") {
            Some(filename) => run(filename.into()).unwrap(),
            _ => repl(),
        },
        _ => unreachable!(),
    }
}
