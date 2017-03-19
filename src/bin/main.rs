extern crate rust_jvm;

use rust_jvm::vm;
use std::path::PathBuf;

const RUNTIME_PATH: &'static str = concat!(env!("OUT_DIR"), "/runtime");

fn main() {
    let class_file = std::env::args().nth(1).unwrap();

    let runtime = vm::Runtime::new(PathBuf::from(RUNTIME_PATH));
    runtime.start(vm::symref::Class { sig: vm::sig::Class::Scalar(class_file) });
}
