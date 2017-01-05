use std::io::{Cursor, Read};

use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub enum ConstantInfo {
    Class { name_index: u16 },
    Fieldref { class_index: u16, name_and_type_index: u16 },
    Methodref { class_index: u16, name_and_type_index: u16 },
    InterfaceMethodref { class_index: u16, name_and_type_index: u16 },
    String { string_index: u16 },
    Integer { value: i32 },
    Float { value: f32 },
    Long { value: i64 },
    Double { value: f64 },
    NameAndType { name_index: u16, descriptor_index: u16 },
    Utf8 { length: u16, value: String }
}

impl ConstantInfo {
    pub fn new(cur: &mut Cursor<Vec<u8>>) -> ConstantInfo {
        let tag = cur.read_u8().unwrap();
        match tag {
            7 => {
                ConstantInfo::Class {
                    name_index: cur.read_u16::<BigEndian>().unwrap()
                }
            },
            9 => {
                ConstantInfo::Fieldref {
                    class_index: cur.read_u16::<BigEndian>().unwrap(),
                    name_and_type_index: cur.read_u16::<BigEndian>().unwrap()
                }
            },
            10 => {
                ConstantInfo::Methodref {
                    class_index: cur.read_u16::<BigEndian>().unwrap(),
                    name_and_type_index: cur.read_u16::<BigEndian>().unwrap()
                }
            },
            11 => {
                ConstantInfo::InterfaceMethodref {
                    class_index: cur.read_u16::<BigEndian>().unwrap(),
                    name_and_type_index: cur.read_u16::<BigEndian>().unwrap()
                }
            },
            8 => {
                ConstantInfo::String {
                    string_index: cur.read_u16::<BigEndian>().unwrap()
                }
            },
            3 => {
                ConstantInfo::Integer {
                    value: cur.read_i32::<BigEndian>().unwrap()
                }
            },
            4 => {
                ConstantInfo::Float {
                    value: cur.read_f32::<BigEndian>().unwrap()
                }
            },
            5 => {
                ConstantInfo::Long {
                    value: cur.read_i64::<BigEndian>().unwrap()
                }
            },
            6 => {
                ConstantInfo::Double {
                    value: cur.read_f64::<BigEndian>().unwrap()
                }
            },
            12 => {
                ConstantInfo::NameAndType {
                    name_index: cur.read_u16::<BigEndian>().unwrap(),
                    descriptor_index: cur.read_u16::<BigEndian>().unwrap()
                }
            },
            1 => {
                let length = cur.read_u16::<BigEndian>().unwrap(); 
                let bytes = vec![0 as u8; length as usize];
                let mut slice = bytes.into_boxed_slice();
                cur.read_exact(&mut slice);
                let bytes = slice.into_vec();
                ConstantInfo::Utf8 {
                    length: length,
                    value: String::from_utf8(bytes).unwrap()
                }
            },
            _ => {
                panic!("Unknown constant type {}", tag);
            }
        }
    }
}

