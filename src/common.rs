pub type AccessFlags = u16;

pub const ACC_PUBLIC:  AccessFlags = 0x0001;
pub const ACC_PRIVATE: AccessFlags = 0x0002;
pub const ACC_PROTECTED: AccessFlags = 0x0004;
pub const ACC_STATIC: AccessFlags = 0x0008;
pub const ACC_FINAL: AccessFlags = 0x0010;
pub const ACC_SYNCHRONIZED: AccessFlags = 0x0020;
pub const ACC_BRIDGE: AccessFlags = 0x0040;
pub const ACC_VARARGS: AccessFlags = 0x0080;
pub const ACC_NATIVE: AccessFlags = 0x0100;
pub const ACC_ABSTRACT: AccessFlags = 0x0400;
pub const ACC_STRICT: AccessFlags = 0x0800;
pub const ACC_SYNTHETIC: AccessFlags = 0x1000;
