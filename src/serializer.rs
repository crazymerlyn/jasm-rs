use std::io::Write;
use std::io::Result;

use class::Class;
use attribute::Attribute;
use method::Method;
use constants::Constants;
use field::Field;
use interface::Interface;

pub trait Serializable {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<()>;
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

impl<T: Serializable> Serializable for Vec<T> {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<()> {
        (self.len() as u16).serialize(out)?;
        for element in self {
            element.serialize(out)?;
        }
        Ok(())
    }
}

impl Serializable for u16 {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<()> {
        out.write(&[(self >> 8) as u8, (self & 0xff) as u8])?;
        Ok(())
    }
}

impl Serializable for u32 {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<()> {
        let b1 = (self >> 24) as u8;
        let b2 = ((self >> 16) & 0xffu32) as u8;
        let b3 = ((self >> 8) & 0xffu32) as u8;
        let b4 = (self & 0xffu32) as u8;
        out.write(&[b1, b2, b3, b4])?;
        Ok(())
    }
}

impl Serializable for Attribute {
    fn serialize<W: Write>(&self, _out: &mut W) -> Result<()> {
        panic!("unimplemented: TODO implement attributes");
    }
}

impl Serializable for Field {
    fn serialize<W: Write>(&self, _out: &mut W) -> Result<()> {
        panic!("unimplemented: TODO implement fields");
    }
}

impl Serializable for Method {
    fn serialize<W: Write>(&self, _out: &mut W) -> Result<()> {
        panic!("unimplemented: TODO implement methods");
    }
}

impl Serializable for Constants {
    fn serialize<W: Write>(&self, _out: &mut W) -> Result<()> {
        panic!("unimplemented: TODO implement constants");
    }
}

impl Serializable for Interface {
    fn serialize<W: Write>(&self, _out: &mut W) -> Result<()> {
        panic!("unimplemented: TODO implement interfaces");
    }
}
