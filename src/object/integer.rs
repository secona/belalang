pub struct Integer {
    value: i64,
}

impl super::Object for Integer {
    fn object_type(&self) -> super::ObjectType {
        super::ObjectType::Integer
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}
