use std::io::{Cursor, Read};

use super::constant_info::ConstantInfo;
use super::field_info::FieldInfo;

use byteorder::{BigEndian, ReadBytesExt};

const MAGIC_VALUE: u32 = 0xCAFEBABE;

#[derive(Debug)]
pub struct Class {
    minor_version: u16,
    major_version: u16,
    constant_pool_len: u16,
    constant_pool: Box<[ConstantInfo]>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_len: u16,
    interfaces: Box<[u16]>,
    fields_len: u16,
    fields: Box<[FieldInfo]>,
    /*
    methods_count: u16,
    methods: Box<[MethodInfo]>,
    attributes_count: u16,
    attributes: Box<[AttributeInfo]>
    */
}

impl Class {
    pub fn new(data: Vec<u8>) -> Class {
        let mut cur = Cursor::new(data);
        assert_eq!(MAGIC_VALUE, cur.read_u32::<BigEndian>().unwrap());
        
        let minor_version = cur.read_u16::<BigEndian>().unwrap();
        let major_version = cur.read_u16::<BigEndian>().unwrap();
        
        let constant_pool_len = cur.read_u16::<BigEndian>().unwrap();
        let mut constant_pool = Vec::with_capacity((constant_pool_len - 1) as usize);
        for i in 1..constant_pool_len {
            let tag = cur.read_u8().unwrap();
            constant_pool.push(ConstantInfo::new(tag, &mut cur));
        }

        let access_flags = cur.read_u16::<BigEndian>().unwrap();
        let this_class = cur.read_u16::<BigEndian>().unwrap();
        let super_class = cur.read_u16::<BigEndian>().unwrap();

        let interfaces_len = cur.read_u16::<BigEndian>().unwrap();
        let mut interfaces = Vec::with_capacity(interfaces_len as usize);
        for i in 0..interfaces_len {
            interfaces.push(cur.read_u16::<BigEndian>().unwrap());
        }

        let fields_len = cur.read_u16::<BigEndian>().unwrap();
        let mut fields = Vec::with_capacity(interfaces_len as usize);
        for i in 0..fields_len {
            fields.push(FieldInfo::new(&mut cur));
        }
        Class {
            minor_version: minor_version,
            major_version: major_version,
            constant_pool_len: constant_pool_len,
            constant_pool: constant_pool.into_boxed_slice(),
            access_flags: access_flags,
            this_class: this_class,
            super_class: super_class,
            interfaces_len: interfaces_len,
            interfaces: interfaces.into_boxed_slice(),
            fields_len: fields_len,
            fields: fields.into_boxed_slice()
        }
    }
}


