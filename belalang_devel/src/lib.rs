use std::fmt::Display;

pub mod ops;

pub trait BelalangType: Display {
    fn type_name(&self) -> &str;
}
