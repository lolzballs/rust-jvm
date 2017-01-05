extern crate byteorder;

use std::env;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;
use std::string::String;

use byteorder::{ByteOrder, BigEndian, ReadBytesExt};

const MAGIC_VALUE: u32 = 0xCAFEBABE;

// TODO: Enum
struct AttributeInfo {
    name_index: u16,
    length: u32,
    info: Box<[u8]>
}

#[derive(Debug)]
enum ConstantInfo {
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
    fn new(tag: u8, cur: &mut Cursor<Vec<u8>>) -> ConstantInfo {
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

struct FieldInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Box<[AttributeInfo]>
}

struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Box<[AttributeInfo]>
}

#[derive(Debug)]
struct Class {
    minor_version: u16,
    major_version: u16,
    constant_pool_len: u16,
    constant_pool: Box<[ConstantInfo]>,
    /*
    access_flags: u16,
    this_class: u16,
    super_class: u16,
    interfaces_len: u16,
    interfaces: Box<[u16]>,
    fields_len: u16,
    fields: Box<[FieldInfo]>,
    methods_count: u16,
    methods: Box<[MethodInfo]>,
    attributes_count: u16,
    attributes: Box<[AttributeInfo]>
    */
}

impl Class {
    fn new(data: Vec<u8>) -> Class {
        let mut cur = Cursor::new(data);
        assert_eq!(MAGIC_VALUE, cur.read_u32::<BigEndian>().unwrap());
        
        let minor_version = cur.read_u16::<BigEndian>().unwrap();
        let major_version = cur.read_u16::<BigEndian>().unwrap();
        let constant_pool_len = cur.read_u16::<BigEndian>().unwrap();
        let mut constant_pool = Vec::with_capacity((constant_pool_len - 1) as usize);
        for i in 1..constant_pool_len {
            let tag = cur.read_u8().unwrap();
            println!("Tag: {}", tag);
            constant_pool.push(ConstantInfo::new(tag, &mut cur));
        }
        Class {
            minor_version: minor_version,
            major_version: major_version,
            constant_pool_len: constant_pool_len,
            constant_pool: constant_pool.into_boxed_slice()
        }
    }
}

fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf);
    buf
}

fn main() {
    let class_file = env::args().nth(1).unwrap();

    let class_bin = read_bin(class_file);
    
    let class = Class::new(class_bin);
    println!("Class: {:?}", class);
}

