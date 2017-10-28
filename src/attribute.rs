use serializer::Serializable;
use std::io::Write;
use std::io::Result;

use exception::Exception;

pub struct Attribute {
    name_index: u16,
    length: u32,
    ty: AttributeType,
}

impl Serializable for Attribute {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<()> {
        self.name_index.serialize(out)?;
        self.length.serialize(out)?;
        self.ty.serialize(out)
    }
}

pub enum AttributeType {
    Constant(u16),
    Code(CodeAttribute),
}

impl Serializable for AttributeType {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<()> {
        match *self {
            AttributeType::Constant(index) => index.serialize(out),
            AttributeType::Code(ref code) => code.serialize(out),
        }
    }
}

pub struct CodeAttribute {
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Vec<u8>,
    pub exceptions: Vec<Exception>,
    pub attributes: Vec<Attribute>,
}

impl Serializable for CodeAttribute {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<()> {
        self.max_stack.serialize(out)?;
        self.max_locals.serialize(out)?;
        self.code.serialize(out)?;
        self.exceptions.serialize(out)?;
        self.attributes.serialize(out)
    }
}
