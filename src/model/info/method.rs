use std::io::Cursor;

use super::{Attribute, Constant};

use byteorder::{BigEndian, ReadBytesExt};

const ACC_PUBLIC: u16 = 0x0001;
const ACC_PRIVATE: u16 = 0x0002;
const ACC_PROTECTED: u16 = 0x0004;
const ACC_STATIC: u16 = 0x0008;
const ACC_FINAL: u16 = 0x0010;
const ACC_SYNCHRONIZED: u16 = 0x0020;
const ACC_BRIDGE: u16 = 0x0040;
const ACC_VARARGS: u16 = 0x0080;
const ACC_NATIVE: u16 = 0x0100;
const ACC_ABSTRACT: u16 = 0x0400;
const ACC_STRICT: u16 = 0x0800;
const ACC_SYNTHETIC: u16 = 0x1000;

#[derive(Debug)]
pub struct Method {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    attributes_count: u16,
    pub attributes: Box<[Attribute]>
}

impl Method {
    pub fn new(constant_pool: &Box<[Constant]>, 
               cur: &mut Cursor<Vec<u8>>) -> Method{
        let access_flags = cur.read_u16::<BigEndian>().unwrap();
        let name_index = cur.read_u16::<BigEndian>().unwrap();
        let descriptor_index = cur.read_u16::<BigEndian>().unwrap();

        let attributes_count = cur.read_u16::<BigEndian>().unwrap();
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(Attribute::new(constant_pool, cur));
        }

        Method {
            access_flags: access_flags,
            name_index: name_index,
            descriptor_index: descriptor_index,
            attributes_count: attributes_count,
            attributes: attributes.into_boxed_slice()
        }
    }
}
