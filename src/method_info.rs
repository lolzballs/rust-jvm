use std::io::{Cursor, Read};

use super::attribute_info::AttributeInfo;
use super::constant_info::ConstantInfo;

use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Box<[AttributeInfo]>
}

impl MethodInfo {
    pub fn new(constant_pool: &Box<[ConstantInfo]>, 
               cur: &mut Cursor<Vec<u8>>) -> MethodInfo {
        let access_flags = cur.read_u16::<BigEndian>().unwrap();
        let name_index = cur.read_u16::<BigEndian>().unwrap();
        let descriptor_index = cur.read_u16::<BigEndian>().unwrap();

        let attributes_count = cur.read_u16::<BigEndian>().unwrap();
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for i in 0..attributes_count {
            attributes.push(AttributeInfo::new(constant_pool, cur));
        }

        MethodInfo {
            access_flags: access_flags,
            name_index: name_index,
            descriptor_index: descriptor_index,
            attributes_count: attributes_count,
            attributes: attributes.into_boxed_slice()
        }
    }
}
