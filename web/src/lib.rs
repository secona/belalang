use belalang::{
    evaluator::{
        builtins::{BuiltinFunction, Builtins},
        object::Object,
        Evaluator,
    },
    lexer::Lexer,
    parser::Parser,
};
use wasm_bindgen::prelude::*;

pub struct Println {
    document: web_sys::Document,
    parent: web_sys::Element,
}

impl Println {
    pub fn new() -> Self {
        let document = web_sys::window().and_then(|win| win.document()).unwrap();
        let parent = document.get_element_by_id("out").expect("should have out");

        Self { document, parent }
    }
}

impl BuiltinFunction for Println {
    fn call(&self, args: Vec<Object>) -> Object {
        let p = self.document.create_element("p").unwrap();

        p.set_text_content(Some(&format!(
            "{}",
            args.iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )));

        let _ = self.parent.append_child(&p);

        Object::Null
    }
}

#[wasm_bindgen]
pub fn run_code(input: String) {
    let lexer = Lexer::new(input.as_bytes());
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().unwrap();

    let mut builtins = Builtins::default();
    builtins.override_builtin("println".into(), Box::new(Println::new()));

    let mut ev = Evaluator::new(program, builtins);
    let _ = ev.evaluate();
}
