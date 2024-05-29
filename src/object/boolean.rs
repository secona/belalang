pub struct Boolean {
    pub value: bool,
}

impl super::ObjectTrait for Boolean {
    fn object_type(&self) -> super::ObjectType {
        super::ObjectType::Boolean
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}
