use serializer::Serializable;
use std::io::Write;
use std::io::Result;

use constants::Constants;
use interface::Interface;
use field::Field;
use method::Method;
use attribute::Attribute;

pub struct Class {
    magic_number: u32,
    minor_version: u16,
    major_version: u16,
    constants: Constants,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces: Vec<Interface>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    attributes: Vec<Attribute>,
}

impl Serializable for Class {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<()> {
        self.magic_number.serialize(out)?;
        self.minor_version.serialize(out)?;
        self.major_version.serialize(out)?;
        self.constants.serialize(out)?;
        self.access_flags.serialize(out)?;
        self.this_class.serialize(out)?;
        self.super_class.serialize(out)?;
        self.interfaces.serialize(out)?;
        self.fields.serialize(out)?;
        self.methods.serialize(out)?;
        self.attributes.serialize(out)?;
        Ok(())
    }
}

