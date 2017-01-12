extern crate rust_jvm;

use rust_jvm::vm;

fn main() {
    let class_file = std::env::args().nth(1).unwrap();

    let runtime = vm::Runtime::new();
    runtime.start(vm::symref::Class { sig: vm::sig::Class::Scalar(class_file) });
}
