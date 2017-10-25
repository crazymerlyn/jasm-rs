use serializer::Serializable;
use std::io::Write;
use std::io::Result;

pub struct Field {
    
}

impl Field {
    pub fn new() -> Field {
        Field {  }
    }
}

impl Serializable for Field {
    fn serialize<W: Write>(&self, _out: &mut W) -> Result<()> {
        panic!("unimplemented: TODO implement fields");
    }
}
