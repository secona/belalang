#![allow(dead_code)]

macro_rules! as_variant {
    ($value:expr, $variant:path) => {
        match $value {
            $variant(x) => Some(x),
            _ => None,
        }
    };
}

pub(crate) use as_variant;

macro_rules! stringify {
    ($value:expr, $expected:expr) => {
        assert_eq!($value.to_string(), $expected);
    }
}

pub (crate) use stringify;

macro_rules! identifier {
    ($value:expr, $expected:expr) => {
        assert_eq!($value.value, $expected);
        assert_eq!($value.token.to_string(), $expected.to_string());
    };
}

pub(crate) use identifier;

macro_rules! expression {
    ($value:expr, $variant:path = $expected:expr) => {
        let v = testing::as_variant!($value, $variant).expect("variant not match");

        assert_eq!(v.value, $expected);
        assert_eq!(v.token.to_string(), $expected.to_string());
    };
}

pub(crate) use expression;

macro_rules! infix {
    ($value:expr, $left_variant:path = $left:expr, $op:expr, $right_variant:path = $right:expr) => {
        let v = testing::as_variant!($value, ast::Expression::InfixExpression)
            .expect("not a(n) ast::Expression::InfixExpression");

        testing::expression!(&*v.left, $left_variant = $left);
        testing::expression!(&*v.right, $right_variant = $right);
        assert_eq!(v.operator, $op.to_string());
    };
}

pub(crate) use infix;
