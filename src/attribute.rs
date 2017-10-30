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
    StackMapTable(Vec<StackMapFrame>),
}

impl Serializable for AttributeType {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<()> {
        match *self {
            AttributeType::Constant(index) => index.serialize(out),
            AttributeType::Code(ref code) => code.serialize(out),
            AttributeType::StackMapTable(ref entries) => entries.serialize(out),
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

pub enum TypeInfo {
    TopVariable,
    Integer,
    Float,
    Long,
    Double,
    Null,
    UninitializedThis,
    Object(u16),
    Uninitialized(u16),
}

const TOP_VARIABLE_INFO: u8 = 0;
const INTEGER_VARIABLE_INFO: u8 = 1;
const FLOAT_VARIABLE_INFO: u8 = 2;
const LONG_VARIABLE_INFO: u8 = 4;
const DOUBLE_VARIABLE_INFO: u8 = 3;
const NULL_VARIABLE_INFO: u8 = 5;
const UNINITIALIZED_THIS_VARIABLE_INFO: u8 = 6;
const OBJECT_VARIABLE_INFO: u8 = 7;
const UNINITIALIZED_VARIABLE_INFO: u8 = 8;

impl Serializable for TypeInfo {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<()> {
        match *self {
            TypeInfo::TopVariable => TOP_VARIABLE_INFO.serialize(out),
            TypeInfo::Integer => INTEGER_VARIABLE_INFO.serialize(out),
            TypeInfo::Float => FLOAT_VARIABLE_INFO.serialize(out),
            TypeInfo::Long => LONG_VARIABLE_INFO.serialize(out),
            TypeInfo::Double => DOUBLE_VARIABLE_INFO.serialize(out),
            TypeInfo::Null => NULL_VARIABLE_INFO.serialize(out),
            TypeInfo::UninitializedThis => UNINITIALIZED_THIS_VARIABLE_INFO.serialize(out),
            TypeInfo::Object(index) => {
                OBJECT_VARIABLE_INFO.serialize(out)?;
                index.serialize(out)
            }
            TypeInfo::Uninitialized(offset) => {
                UNINITIALIZED_VARIABLE_INFO.serialize(out)?;
                offset.serialize(out)
            }
        }
    }
}

pub enum StackMapFrame {
    Same,
    SameLocals1StackItem(u8, TypeInfo),
    SameLocals1StackItemExtended(u16, TypeInfo),
    Chop(u8, u16),
    SameExtended(u16),
    Append(u16, Vec<TypeInfo>),
    Full(u16, Vec<TypeInfo>, Vec<TypeInfo>),
}

const EXTENDED_STACK_ITEM_TAG: u8 = 247;
const SAME_FRAME_TAG: u8 = 251;
const FULL_FRAME_TAG: u8 = 255;

impl Serializable for StackMapFrame {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<()> {
        match *self {
            StackMapFrame::Same => 0u8.serialize(out),
            StackMapFrame::SameLocals1StackItem(ty, ref info) => {
                ty.serialize(out)?;
                info.serialize(out)
            }
            StackMapFrame::SameLocals1StackItemExtended(offset, ref info) => {
                EXTENDED_STACK_ITEM_TAG.serialize(out)?;
                offset.serialize(out)?;
                info.serialize(out)
            }
            StackMapFrame::Chop(n_vars, offset) => {
                (SAME_FRAME_TAG - n_vars).serialize(out)?;
                offset.serialize(out)
            }
            StackMapFrame::SameExtended(offset) => {
                SAME_FRAME_TAG.serialize(out)?;
                offset.serialize(out)
            }
            StackMapFrame::Append(offset, ref infos) => {
                (SAME_FRAME_TAG + infos.len() as u8).serialize(out)?;
                offset.serialize(out)?;
                for info in infos {
                    info.serialize(out)?;
                }
                Ok(())
            }
            StackMapFrame::Full(offset, ref locals, ref stack) => {
                FULL_FRAME_TAG.serialize(out)?;
                offset.serialize(out)?;
                locals.serialize(out)?;
                stack.serialize(out)
            }
        }
    }
}
