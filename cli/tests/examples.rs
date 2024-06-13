use std::fs;

#[test]
fn examples_work() {
    for filename in fs::read_dir("examples").unwrap() {
        let filename = filename.unwrap().path();
        belalang_cli::run_file(filename).unwrap();
    }
}

