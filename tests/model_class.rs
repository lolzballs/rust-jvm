#[cfg(test)]

extern crate rust_jvm;

use self::rust_jvm::model::Class;
use self::rust_jvm::model::info::Constant;
use std::fs::File;
use std::io::Read;

#[test]
fn test_load_class() {
    let mut file = File::open("test_data/model_class/Test.class").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf);

    let class = Class::new(buf);
    println!("{:#?}", class);
    let ref constant = class.constant_pool[7 as usize];
    match *constant {
        Constant::Long { value } => assert_eq!(value, 12312312i64),
        _ => assert!(false),
    }
}
