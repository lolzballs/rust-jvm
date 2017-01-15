#[cfg(test)]

extern crate rust_jvm;

use std::fs;
use std::num::Wrapping;
use rust_jvm::vm::ClassLoader;
use rust_jvm::vm::sig;
use rust_jvm::vm::symref;
use rust_jvm::vm::value::Value;

#[test]
fn test_instance() {
    let path = fs::canonicalize("./runtime").unwrap();

    let res = std::env::set_current_dir("test_data/instance");
    assert!(res.is_ok());

    let mut class_loader = ClassLoader::new(path);
    let class = class_loader.resolve_class(&sig::Class::Scalar(String::from("Instance")));

    let sig = sig::Method {
        name: String::from("setAndGetValue"),
        params: vec![sig::Type::Int],
        return_type: Some(sig::Type::Int),
    };

    let symref = symref::Method {
        class: class.symref.clone(),
        sig: sig,
    };

    let method = class.find_method(&mut class_loader, &symref);
    let mut args = vec![];
    args.push(Value::Int(Wrapping(69)));
    let ret = method.invoke(&class, &mut class_loader, Some(args)).unwrap();
    match ret {
        Value::Int(value) => assert_eq!(value.0, 69),
        _ => panic!("Expected Int with value 69, got {:?}", ret),
    }
}
