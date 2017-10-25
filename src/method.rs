use serializer::Serializable;
use std::io::Write;
use std::io::Result;

pub struct Method {
    
}

impl Method {
    pub fn new() -> Method {
        Method {  }
    }
}

impl Serializable for Method {
    fn serialize<W: Write>(&self, _out: &mut W) -> Result<()> {
        panic!("unimplemented: TODO implement methods");
    }
}
