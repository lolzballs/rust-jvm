extern crate byteorder;

mod class;
mod constant_info;
mod field_info;
mod method_info;
mod attribute_info;

use std::env;
use std::fs::File;
use std::io::{Cursor, Read};
use std::path::Path;

use byteorder::{BigEndian, ReadBytesExt};

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

