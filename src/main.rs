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
    let class = vm::class::Class::new(class_symref.clone(), None, constant_pool, class);
    class.initialize();

    use vm::sig;
    use vm::symref;
    let string_ty = sig::Type::Reference(sig::Class::Scalar(String::from("java/lang/String")));
    let string_array_ty = sig::Type::Reference(sig::Class::Array(Box::new(string_ty)));
    let main_sig = sig::Method {
        name: String::from("main"),
        params: vec![string_array_ty],
        return_type: None,
    };
    let main_symref = symref::Method {
        class: class_symref,
        sig: main_sig,
    };

    let method = class.find_method(&main_symref);
    method.invoke(&class, None);
}
