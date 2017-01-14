#[cfg(test)]

extern crate rust_jvm;

use std::fs::File;
use std::io::Read;
use rust_jvm::model::Class;
use rust_jvm::model::info::Constant;

#[test]
fn test_load_class() {
    std::env::set_current_dir("test_data/model_class");
    let mut file = File::open("Test.class").unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf);

    let class = Class::new(buf);
    println!("{:#?}", class);
    let ref constant = class.constant_pool[7 as usize];
    match *constant {
        Constant::Long { value } => assert_eq!(value, 12312312i64),
        _ => panic!("Expected Long with value 12312312, got {:?}", constant),
    }
}
