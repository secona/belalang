use crate::{evaluator, lexer, parser};

pub fn test_eval(
    input: String,
) -> Result<evaluator::object::Object, evaluator::error::EvaluatorError> {
    let input = input.as_bytes().into();
    let lexer = lexer::Lexer::new(input);
    let mut parser = parser::Parser::new(lexer);
    let program = parser.parse_program().expect("parser errors");

    let mut ev = evaluator::Evaluator::default();
    return ev.eval_program(program);
}

macro_rules! as_variant {
    ($value:expr, $variant:path) => {
        if let $variant(x) = $value {
            x
        } else {
            panic!("unmatching variant! got={}", stringify!($variant));
        }
    };
}

pub(crate) use as_variant;

macro_rules! eval {
    ($input:expr, $variant:path = $expected:expr) => {
        let evaluated = testing::test_eval($input.into());

        match evaluated {
            Ok($variant(value)) => assert_eq!(value, $expected),
            Ok(unexpected) => panic!("got unexpected object. got={}", unexpected),
            Err(err) => panic!("got errors instead. got={}", err),
        }
    };
    ($input:expr, $variant:pat) => {
        let evaluated = testing::test_eval($input.into());

        match evaluated {
            Ok(obj) => matches!(obj, $variant),
            Err(err) => panic!("got errors instead. got={}", err),
        }
    };
    ($input:expr, Err => $expected:expr) => {
        let evaluated = testing::test_eval($input.into());

        match evaluated {
            Ok(unexpected) => panic!("got ok instead. got={}", unexpected),
            Err(err) => assert_eq!(err.to_string(), $expected),
        }
    };
}

pub(crate) use eval;
