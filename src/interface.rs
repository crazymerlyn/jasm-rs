use serializer::Serializable;
use std::io::Write;
use std::io::Result;

pub struct Interface {
    
}

impl Interface {
    pub fn new() -> Interface {
        Interface {  }
    }
}

impl Serializable for Interface {
    fn serialize<W: Write>(&self, _out: &mut W) -> Result<()> {
        panic!("unimplemented: TODO implement interfaces");
    }
}
