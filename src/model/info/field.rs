use std::io::Cursor;

use super::{Attribute, Constant};

use byteorder::{BigEndian, ReadBytesExt};

pub const ACC_PUBLIC: u16 = 0x0001;
pub const ACC_PRIVATE: u16 = 0x0002;
pub const ACC_PROTECTED: u16 = 0x0004;
pub const ACC_STATIC: u16 = 0x0008;
pub const ACC_FINAL: u16 = 0x0010;
pub const ACC_VOLATILE: u16 = 0x0040;
pub const ACC_TRANSIENT: u16 = 0x0080;
pub const ACC_SYNTEHTIC: u16 = 0x1000;
pub const ACC_ENUM: u16 = 0x4000;

#[derive(Debug)]
pub struct Field {
    pub access_flags: u16,
    pub name_index: u16,
    pub descriptor_index: u16,
    attributes_count: u16,
    pub attributes: Box<[Attribute]>,
}

impl Field {
    pub fn new(constant_pool: &Box<[Constant]>, cur: &mut Cursor<Vec<u8>>) -> Field {
        let access_flags = cur.read_u16::<BigEndian>().unwrap();
        let name_index = cur.read_u16::<BigEndian>().unwrap();
        let descriptor_index = cur.read_u16::<BigEndian>().unwrap();

        let attributes_count = cur.read_u16::<BigEndian>().unwrap();
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(Attribute::new(constant_pool, cur));
        }

        Field {
            access_flags: access_flags,
            name_index: name_index,
            descriptor_index: descriptor_index,
            attributes_count: attributes_count,
            attributes: attributes.into_boxed_slice(),
        }
    }
}
