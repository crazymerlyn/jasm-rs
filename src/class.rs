use constants::Constants;
use interface::Interface;
use field::Field;
use method::Method;
use attribute::Attribute;

pub struct Class {
    pub magic_number: u32,
    pub minor_version: u16,
    pub major_version: u16,
    pub constants: Constants,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<Interface>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub attributes: Vec<Attribute>,
}
