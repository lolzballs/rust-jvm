use std::io::{Cursor, Read};

use super::constant_info::ConstantInfo;
use super::field_info::FieldInfo;
use super::method_info::MethodInfo;
use super::attribute_info::AttributeInfo;

use byteorder::{BigEndian, ReadBytesExt};

const MAGIC_VALUE: u32 = 0xCAFEBABE;

#[derive(Debug)]
pub struct Class {
    minor_version: u16,
    major_version: u16,
    constant_pool_count: u16,
    constant_pool: Box<[ConstantInfo]>,
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_count: u16,
    interfaces: Box<[u16]>,
    fields_count: u16,
    fields: Box<[FieldInfo]>,
    methods_count: u16,
    methods: Box<[MethodInfo]>,
    attributes_count: u16,
    attributes: Box<[AttributeInfo]>
}

impl Class {
    pub fn new(data: Vec<u8>) -> Class {
        let mut cur = Cursor::new(data);
        assert_eq!(MAGIC_VALUE, cur.read_u32::<BigEndian>().unwrap());
        
        let minor_version = cur.read_u16::<BigEndian>().unwrap();
        let major_version = cur.read_u16::<BigEndian>().unwrap();
        
        let constant_pool_count = cur.read_u16::<BigEndian>().unwrap();
        let mut constant_pool = Vec::with_capacity((constant_pool_count - 1) as usize);
        for i in 1..constant_pool_count {
            constant_pool.push(ConstantInfo::new(&mut cur));
        }
        let constant_pool = constant_pool.into_boxed_slice();

        let access_flags = cur.read_u16::<BigEndian>().unwrap();
        let this_class = cur.read_u16::<BigEndian>().unwrap();
        let super_class = cur.read_u16::<BigEndian>().unwrap();

        let interfaces_count = cur.read_u16::<BigEndian>().unwrap();
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);
        for i in 0..interfaces_count {
            interfaces.push(cur.read_u16::<BigEndian>().unwrap());
        }

        let fields_count = cur.read_u16::<BigEndian>().unwrap();
        let mut fields = Vec::with_capacity(interfaces_count as usize);
        for i in 0..fields_count {
            fields.push(FieldInfo::new(&constant_pool, &mut cur));
        }

        let methods_count = cur.read_u16::<BigEndian>().unwrap();
        let mut methods = Vec::with_capacity(methods_count as usize);
        for i in 0..methods_count {
            methods.push(MethodInfo::new(&constant_pool, &mut cur));
        }

        let attributes_count = cur.read_u16::<BigEndian>().unwrap();
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for i in 0..attributes_count {
            attributes.push(AttributeInfo::new(&constant_pool, &mut cur));
        }
        Class {
            minor_version: minor_version,
            major_version: major_version,
            constant_pool_count: constant_pool_count,
            constant_pool: constant_pool,
            access_flags: access_flags,
            this_class: this_class,
            super_class: super_class,
            interfaces_count: interfaces_count,
            interfaces: interfaces.into_boxed_slice(),
            fields_count: fields_count,
            fields: fields.into_boxed_slice(),
            methods_count: methods_count,
            methods: methods.into_boxed_slice(),
            attributes_count: attributes_count,
            attributes: attributes.into_boxed_slice()
        }
    }
}

