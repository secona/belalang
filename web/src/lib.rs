use belalang::{
    evaluator::{
        builtins::{self, Builtins},
        object::Object,
        Evaluator,
    },
    lexer::Lexer,
    parser::Parser,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    let mut fns = builtins::BUILTIN_FUNCTIONS.lock().unwrap();

    fns.insert(
        "println".into(),
        Box::new(|args| {
            let document = web_sys::window().and_then(|win| win.document()).unwrap();
            let parent = document.get_element_by_id("out").expect("should have out");

            let p = document.create_element("p").unwrap();

            p.set_text_content(Some(&format!(
                "{}",
                args.iter()
                    .map(|arg| arg.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            )));

            let _ = parent.append_child(&p);

            Object::Null
        }),
    );
}

#[wasm_bindgen]
pub fn run_code(input: String) {
    let lexer = Lexer::new(input.as_bytes());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();

    let builtins = Builtins::default();
    let mut ev = Evaluator::new(builtins);
    let _ = ev.eval_program(program);
}
