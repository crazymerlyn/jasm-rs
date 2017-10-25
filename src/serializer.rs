use std::io::Write;
use std::io::Result;

pub trait Serializable {
    fn serialize<W: Write>(&self, out: &mut W) -> Result<()>;
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
