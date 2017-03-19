#[cfg(test)]

extern crate rust_jvm;

use std::num::Wrapping;
use rust_jvm::vm::ClassLoader;
use rust_jvm::vm::sig;
use rust_jvm::vm::symref;
use rust_jvm::vm::value::Value;

const RUNTIME_PATH: &'static str = concat!(env!("OUT_DIR"), "/runtime");

#[test]
fn test_instance() {
    let mut class_loader = ClassLoader::new(vec!["test_data/instance".into(), RUNTIME_PATH.into()]);
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

    let method = class.find_method(&mut class_loader, &symref).borrow();
    let mut args = vec![];
    args.push(Value::Int(Wrapping(69)));
    let ret = method.invoke(&class, &mut class_loader, Some(args)).unwrap();
    match ret {
        Value::Int(value) => assert_eq!(value.0, 69),
        _ => panic!("Expected Int with value 69, got {:?}", ret),
    }
}
