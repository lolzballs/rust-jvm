extern crate byteorder;

mod model;
mod vm;

use vm::Runtime;

fn main() {
    let class_file = std::env::args().nth(1).unwrap();

    let runtime = Runtime::new();
    runtime.start(vm::symref::Class { sig: vm::sig::Class::Scalar(class_file) });
}
