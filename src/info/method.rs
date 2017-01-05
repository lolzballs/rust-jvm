use std::io::{Cursor, Read};

use super::{Attribute, Constant};

use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub struct Method {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Box<[Attribute]>
}

impl Method{
    pub fn new(constant_pool: &Box<[Constant]>, 
               cur: &mut Cursor<Vec<u8>>) -> Method{
        let access_flags = cur.read_u16::<BigEndian>().unwrap();
        let name_index = cur.read_u16::<BigEndian>().unwrap();
        let descriptor_index = cur.read_u16::<BigEndian>().unwrap();

        let attributes_count = cur.read_u16::<BigEndian>().unwrap();
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for i in 0..attributes_count {
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
