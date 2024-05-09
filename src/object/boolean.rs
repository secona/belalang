pub struct Boolean {
    value: bool,
}

impl super::Object for Boolean {
    fn object_type(&self) -> super::ObjectType {
        super::ObjectType::Boolean
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}
