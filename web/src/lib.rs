use belalang_core::{lexer::Lexer, parser::Parser};
use belalang_eval::{
    builtins::{self, Builtins},
    evaluator::Evaluator,
    object::Object,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn println(value: &str);
}

#[wasm_bindgen(start)]
pub fn start() {
    let mut fns = builtins::BUILTIN_FUNCTIONS.lock().unwrap();

    fns.insert(
        "println".into(),
        Box::new(|args| {
            println(
                &args
                    .iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(" "),
            );

            Object::Null
        }),
    );
}

#[wasm_bindgen]
pub fn run_code(input: String) {
    let lexer = Lexer::new(input.as_bytes());
    let mut parser = Parser::new(lexer);

    match parser.parse_program() {
        Ok(program) => {
            let builtins = Builtins::default();
            let mut ev = Evaluator::new(builtins);

            if let Err(err) = ev.eval_program(program) {
                println(&err.to_string());
            }
        }
        Err(err) => {
            println(&err.to_string());
            return;
        }
    }
}
