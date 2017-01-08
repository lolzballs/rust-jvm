use std::io::Cursor;

use super::info::{Attribute, Constant, Field, Method};

use byteorder::{BigEndian, ReadBytesExt};

const MAGIC_VALUE: u32 = 0xCAFEBABE;

#[derive(Debug)]
pub struct Class {
    pub minor_version: u16,
    pub major_version: u16,
    pub constant_pool_count: u16,
    pub constant_pool: Box<[Constant]>,
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces_count: u16,
    pub interfaces: Box<[u16]>,
    pub fields_count: u16,
    pub fields: Box<[Field]>,
    pub methods_count: u16,
    pub methods: Box<[Method]>,
    pub attributes_count: u16,
    pub attributes: Box<[Attribute]>,
}

impl Class {
    pub fn new(data: Vec<u8>) -> Class {
        let mut cur = Cursor::new(data);
        assert_eq!(MAGIC_VALUE, cur.read_u32::<BigEndian>().unwrap());

        let minor_version = cur.read_u16::<BigEndian>().unwrap();
        let major_version = cur.read_u16::<BigEndian>().unwrap();

        let constant_pool_count = cur.read_u16::<BigEndian>().unwrap();
        let mut constant_pool = Vec::with_capacity((constant_pool_count - 1) as usize);
        let mut i = 1;
        while i < constant_pool_count {
            let constant = Constant::new(&mut cur);
            match constant {
                Constant::Long { .. } |
                Constant::Double { .. } => {
                    i += 1; // Longs and Doubles take up two slots...
                    constant_pool.push(constant);
                    constant_pool.push(Constant::Nothing);
                }
                _ => constant_pool.push(constant),
            };
            i += 1;
        }
        let constant_pool = constant_pool.into_boxed_slice();

        let access_flags = cur.read_u16::<BigEndian>().unwrap();
        let this_class = cur.read_u16::<BigEndian>().unwrap();
        let super_class = cur.read_u16::<BigEndian>().unwrap();

        let interfaces_count = cur.read_u16::<BigEndian>().unwrap();
        let mut interfaces = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..interfaces_count {
            interfaces.push(cur.read_u16::<BigEndian>().unwrap());
        }

        let fields_count = cur.read_u16::<BigEndian>().unwrap();
        let mut fields = Vec::with_capacity(interfaces_count as usize);
        for _ in 0..fields_count {
            fields.push(Field::new(&constant_pool, &mut cur));
        }

        let methods_count = cur.read_u16::<BigEndian>().unwrap();
        let mut methods = Vec::with_capacity(methods_count as usize);
        for _ in 0..methods_count {
            methods.push(Method::new(&constant_pool, &mut cur));
        }

        let attributes_count = cur.read_u16::<BigEndian>().unwrap();
        let mut attributes = Vec::with_capacity(attributes_count as usize);
        for _ in 0..attributes_count {
            attributes.push(Attribute::new(&constant_pool, &mut cur));
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
            attributes: attributes.into_boxed_slice(),
        }
    }
}
