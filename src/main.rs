extern crate byteorder;

mod class;
mod constant_info;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

// TODO: Enum
struct AttributeInfo {
    name_index: u16,
    length: u32,
    info: Box<[u8]>
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

