extern crate byteorder;

mod model;
mod vm;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use model::class;

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
    let constant_pool = vm::constant_pool::ConstantPool::new(&class.constant_pool);
    let class_sig = vm::sig::Class::new("Test");
    let class_symref = vm::symref::Class { sig: class_sig };

    println!("{:#?}", vm::class::Class::new(class_symref, None, constant_pool, class));
}

