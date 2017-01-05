extern crate byteorder;

mod class;
mod constant_info;
mod field_info;

use std::env;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;

use byteorder::{BigEndian, ReadBytesExt};

// TODO: Enum
#[derive(Debug)]
struct AttributeInfo {
    name_index: u16,
    length: u32,
    info: Box<[u8]>
}

impl AttributeInfo {
    pub fn new(cur: &mut Cursor<Vec<u8>>) -> AttributeInfo {
        let name_index = cur.read_u16::<BigEndian>().unwrap();
        let length = cur.read_u32::<BigEndian>().unwrap();

        let bytes = vec![0 as u8; length as usize];
        let mut slice = bytes.into_boxed_slice();
        cur.read_exact(&mut slice);
        
        AttributeInfo {
            name_index: name_index,
            length: length,
            info: slice
        }
    }
}

struct MethodInfo {
    access_flags: u16,
    name_index: u16,
    descriptor_index: u16,
    attributes_count: u16,
    attributes: Box<[AttributeInfo]>
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
    
    let class = class::Class::new(class_bin);
    println!("Class: {:#?}", class);
}

