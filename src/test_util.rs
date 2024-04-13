pub struct ToStringTest<T: ToString> {
    pub obj: T,
    pub exp: String,
}

impl<T: ToString> ToStringTest<T> {
    pub fn test(&self) {
        assert_eq!(self.obj.to_string(), self.exp);
    }
}
