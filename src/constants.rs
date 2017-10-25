use serializer::Serializable;
use std::io::Write;
use std::io::Result;


pub struct Constants {
    inner: Vec<Constant>,
    size: u16,
}

impl Serializable for Constants {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<()> {
        self.size.serialize(out)?;
        for constant in &self.inner {
            constant.serialize(out)?;
        }
        Ok(())
    }
}

impl Constants {
    pub fn new() -> Constants {
        Constants {
            inner: vec![],
            size: 1,
        }
    }
}

pub struct Constant {
    ty: ConstantType,
    index: u16,
}

impl Serializable for Constant {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<()> {
        self.ty.serialize(out)
    }
}

pub enum ConstantType {
    Utf8(String),
    Integer(u32),
    Float(u32),
    Long(u32, u32),
    Double(u32, u32),
    Class(u16),
    String(u16),
    Fieldref(u16, u16),
    Methodref(u16, u16),
    InterfaceMethodref(u16, u16),
    NameAndType(u16, u16),
    MethodHandle(u8, u16),
    MethodType(u16),
    InvokeDynamic(u16, u16),
}

const CONSTANT_UTF8_TAG: u16 = 1;
const CONSTANT_INTEGER_TAG: u16 = 3;
const CONSTANT_FLOAT_TAG: u16 = 4;
const CONSTANT_LONG_TAG: u16 = 5;
const CONSTANT_DOUBLE_TAG: u16 = 6;
const CONSTANT_CLASS_TAG: u16 = 7;
const CONSTANT_STRING_TAG: u16 = 8;
const CONSTANT_FIELDREF_TAG: u16 = 9;
const CONSTANT_METHODREF_TAG: u16 = 10;
const CONSTANT_INTERFACE_METHODREF_TAG: u16 = 11;
const CONSTANT_NAME_AND_TYPE_TAG: u16 = 12;
const CONSTANT_METHOD_HANDLE_TAG: u16 = 15;
const CONSTANT_METHOD_TYPE_TAG: u16 = 16;
const CONSTANT_INVOKE_DYNAMIC_TAG: u16 = 18;


impl Serializable for ConstantType {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<()> {
        match *self {
            ConstantType::Utf8(ref s) => {
                CONSTANT_UTF8_TAG.serialize(out)?;
                modified_utf8(s).serialize(out)
            }
            _ => Ok(())
        }
    }
}

fn modified_utf8(s: &str) -> Vec<u8> {
    let mut res = vec![];
    for ch in s.chars() {
        let mut val: u32 = ch as u32;

        if val > 0 && val < 0x80 {
            res.push(val as u8);
        } else if val == 0 || val < 0x800 {
            let b1 = 0xc0u8 + (val >> 6) as u8;
            let b2 = 0x80u8 + (val & 0x3f) as u8;
            res.extend(&[b1, b2]);
        } else if val <= 0xffff {
            let b1 = 0xe0 + (val >> 12) as u8;
            let b2 = 0x80 + ((val >> 6) & 0x3f) as u8;
            let b3 = 0x80 + (val & 0x3f) as u8;
            res.extend(&[b1, b2, b3]);
        } else if val <= 0xfffff {
            val -= 0x10000;
            res.push(0b11101101);
            let b1 = 0xa0 + (val >> 16) as u8;
            let b2 = 0x80 + ((val >> 10) & 0x3f) as u8;
            res.extend(&[b1, b2]);
            res.push(0b11101101);
            let b3 = 0xb0 + ((val >> 6) & 0xf) as u8;
            let b4 = 0x80 + (val & 0x3f) as u8;
            res.extend(&[b3, b4]);
        } else {
            panic!("invalid unicode scalar value: {}", val);
        }
    }
    res
}
