use serializer::Serializable;
use std::io::Write;
use std::io::Result;


pub struct Constants {
}

impl Constants {
    pub fn new() -> Constants {
        Constants {  }
    }
}

impl Serializable for Constants {
    fn serialize<W: Write>(&self, _out: &mut W) -> Result<()> {
        panic!("unimplemented: TODO implement constants");
    }
}
