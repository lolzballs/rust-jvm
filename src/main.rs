extern crate byteorder;

mod model;
mod vm;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use model::class;
use vm::constant_pool;

fn read_bin<P: AsRef<Path>>(path: P) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf);
    buf
}

fn main() {
    let class_file = env::args().nth(1).unwrap();

    let class_bin = read_bin(&class_file);
    
    let class = class::Class::new(class_bin);

    println!("{:#?}", class);
    println!("{:#?}", constant_pool::ConstantPool::new(&class.constant_pool));
}

