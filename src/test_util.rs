use crate::ast;

pub struct ToStringTest<T: ToString> {
    pub obj: T,
    pub exp: String,
}

impl<T: ToString> ToStringTest<T> {
    pub fn test(&self) {
        assert_eq!(self.obj.to_string(), self.exp);
    }
}

pub fn test_identifier(expr: &dyn ast::Expression, value: String) {
    let ident = expr
        .downcast_ref::<ast::Identifier>()
        .expect("not a(n) ast::Identifier");

    assert_eq!(ident.value, value);
    assert_eq!(ident.token.to_string(), value);
}
