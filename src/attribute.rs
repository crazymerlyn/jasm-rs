use serializer::Serializable;
use std::io::Write;
use std::io::Result;

pub struct Attribute {
    
}

impl Attribute {
    pub fn new() -> Attribute {
        Attribute {  }
    }
}

impl Serializable for Attribute {
    fn serialize<W: Write>(&self, _out: &mut W) -> Result<()> {
        panic!("unimplemented: TODO implement attributes");
    }
}
