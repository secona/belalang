use belalang::evaluator::object;

#[macro_use]
mod common;

#[test]
fn integer() {
    eval!("5;", object::Object::Integer = 5);
    eval!("1209;", object::Object::Integer = 1209);

    eval!("-123;", object::Object::Integer = -123);
    eval!("--123;", object::Object::Integer = 123);
    eval!("---123;", object::Object::Integer = -123);

    eval!("12 * 3;", object::Object::Integer = 36);
    eval!("12 / 3 + 1;", object::Object::Integer = 5);
    eval!("(5 + 1) / 2;", object::Object::Integer = 3);
    eval!("5 * -2;", object::Object::Integer = -10);
    eval!("-5 * -2;", object::Object::Integer = 10);
    eval!("5 % 2;", object::Object::Integer = 1);
}

#[test]
fn boolean() {
    eval!("true;", object::Object::Boolean = true);
    eval!("false;", object::Object::Boolean = false);

    eval!("!true;", object::Object::Boolean = false);
    eval!("!!false;", object::Object::Boolean = false);
    eval!("!!!false;", object::Object::Boolean = true);
    eval!("!!!!true;", object::Object::Boolean = true);

    eval!("1 == 1;", object::Object::Boolean = true);
    eval!("2 != 1;", object::Object::Boolean = true);
    eval!("2 == 1;", object::Object::Boolean = false);
    eval!("2 * 4 == 8;", object::Object::Boolean = true);
    eval!("-1 < 1;", object::Object::Boolean = true);
    eval!("1 < 1;", object::Object::Boolean = false);
    eval!("1 - 2 < 1;", object::Object::Boolean = true);
    eval!("1 + 2 > 1;", object::Object::Boolean = true);

    eval!("true == true;", object::Object::Boolean = true);
    eval!("false == false;", object::Object::Boolean = true);
    eval!("true == false;", object::Object::Boolean = false);
    eval!("true != false;", object::Object::Boolean = true);
    eval!("1 < 2 == true;", object::Object::Boolean = true);

    eval!("2 <= 1;", object::Object::Boolean = false);
    eval!("2 <= 2;", object::Object::Boolean = true);
    eval!("2 <= 3;", object::Object::Boolean = true);

    eval!("2 >= 1;", object::Object::Boolean = true);
    eval!("2 >= 2;", object::Object::Boolean = true);
    eval!("2 >= 3;", object::Object::Boolean = false);
}

#[test]
fn r#if() {
    eval!("if (true) { 1 }", object::Object::Integer = 1);
    eval!("if (false) { 1 } else { 2 }", object::Object::Integer = 2);

    eval!(
        "if (1 < 2) { true } else { false }",
        object::Object::Boolean = true
    );
    eval!(
        "if (1 > 2) { true } else { false }",
        object::Object::Boolean = false
    );
    eval!(
        "if (1 + 2 == 3) { 1 + 2 } else { false }",
        object::Object::Integer = 3
    );
    eval!(
        "if (1) { true } else { false }",
        object::Object::Boolean = false
    );

    eval!("if (false) { true }", object::Object::Null);
}

#[test]
fn error_handling() {
    eval!(
        "5 + true;",
        Err => "unknown operator: 5 + true"
    );
    eval!(
        "if (1 < true) { 10 }",
        Err => "unknown operator: 1 < true"
    );
    eval!(
        "true + false;",
        Err => "unknown operator: true + false"
    );
    eval!(
        "4; true - true; 5;",
        Err => "unknown operator: true - true"
    );
    eval!(
        "b;",
        Err => "unknown variable: b"
    );
}

#[test]
fn variables() {
    eval!("a := 5; a;", object::Object::Integer = 5);
    eval!("a := 5 * 10; a;", object::Object::Integer = 50);
    eval!("a := 10; b := a; b;", object::Object::Integer = 10);

    eval!(
        "a := 1; b := 1; c := a + b * 2; c;",
        object::Object::Integer = 3
    );
}

#[test]
fn assignment_ops() {
    eval!("a := 10; a += 1; a;", object::Object::Integer = 11);
    eval!("a := 10; a -= 1; a;", object::Object::Integer = 9);
    eval!("a := 10; a *= 2; a;", object::Object::Integer = 20);
    eval!("a := 10; a /= 2; a;", object::Object::Integer = 5);
    eval!("a := 10; a %= 3; a;", object::Object::Integer = 1);
}

#[test]
fn logical_ops() {
    eval!("true && true;", object::Object::Boolean = true);
    eval!("true && false;", object::Object::Boolean = false);
    eval!("false && true;", object::Object::Boolean = false);
    eval!("false && false;", object::Object::Boolean = false);

    eval!("true || true;", object::Object::Boolean = true);
    eval!("true || false;", object::Object::Boolean = true);
    eval!("false || true;", object::Object::Boolean = true);
    eval!("false || false;", object::Object::Boolean = false);
}
