use crate::evaluator::object::Object;

pub struct Println;

impl super::BuiltinFunction for Println {
    fn call(&self, args: Vec<Object>) -> Object {
        println!(
            "{}",
            args.iter()
                .map(|arg| arg.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        );

        Object::Null
    }
}
