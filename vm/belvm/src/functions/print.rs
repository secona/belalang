use crate::objects::{BelalangObject, integer::BelalangInteger};

pub fn belalang_print(args: &[Box<dyn BelalangObject>]) -> Box<dyn BelalangObject> {
    println!("{}", args.first().unwrap());
    Box::new(BelalangInteger::new(0))
}
