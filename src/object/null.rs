pub struct Null {}

impl super::Object for Null {
    fn object_type(&self) -> super::ObjectType {
        super::ObjectType::Null
    }

    fn inspect(&self) -> String {
        "null".into()
    }
}
