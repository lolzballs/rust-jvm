//! Represents a JVM method_info structure, which describes a method.

use std::io::Cursor;

use super::{Attribute, Constant};

use byteorder::{BigEndian, ReadBytesExt};

pub const ACC_PUBLIC: u16 = 0x0001;
pub const ACC_PRIVATE: u16 = 0x0002;
pub const ACC_PROTECTED: u16 = 0x0004;
pub const ACC_STATIC: u16 = 0x0008;
pub const ACC_FINAL: u16 = 0x0010;
pub const ACC_SYNCHRONIZED: u16 = 0x0020;
pub const ACC_BRIDGE: u16 = 0x0040;
pub const ACC_VARARGS: u16 = 0x0080;
pub const ACC_NATIVE: u16 = 0x0100;
pub const ACC_ABSTRACT: u16 = 0x0400;
pub const ACC_STRICT: u16 = 0x0800;
pub const ACC_SYNTHETIC: u16 = 0x1000;

/// The `Method` type represents a method_info, which describes a method.
#[derive(Debug)]
pub struct Method {
    /// A mask of flags used to denote access permission to and properties of this method.
    pub access_flags: u16,
    /// The index to the item in the containing `ClassFile`s constant pool which contains
    /// the method's name. The `Constant` should be of type `Constant::Utf8`.
    pub name_index: u16,
    /// The index to the item in the containing `ClassFile`s constant pool which contains
    /// the method's descriptor. The `Constant` should be of type `Constant::Utf8`.
    pub descriptor_index: u16,
    /// The number of attributes that this `Method` contains.
    attributes_count: u16,
    pub attributes: Box<[Attribute]>,
}

impl Method {
    pub fn new(constant_pool: &Box<[Constant]>, cur: &mut Cursor<Vec<u8>>) -> Method {
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
            attributes: attributes.into_boxed_slice(),
        }
    }
}
