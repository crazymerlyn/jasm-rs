use serializer::Serializable;
use attribute::Attribute;
use common::AccessFlags;

use std::io::Write;
use std::io::Result;

pub struct Method {
    access_flags: AccessFlags,
    name_index: u16,
    desc_index: u16,
    attributes: Vec<Attribute>,
}

impl Method {
}

impl Serializable for Method {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<()> {
        self.access_flags.serialize(out)?;
        self.name_index.serialize(out)?;
        self.desc_index.serialize(out)?;
        self.attributes.serialize(out)
    }
}
