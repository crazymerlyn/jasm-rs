use serializer::Serializable;
use std::io::Write;
use std::io::Result;

pub struct Exception {
}

impl Serializable for Exception {
    fn serialize<W: Write>(&self, _out: &mut W) -> Result<()> {
        panic!("unimplemented: TODO implement fields");
    }
}
