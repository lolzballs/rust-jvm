#![allow(non_snake_case)]

extern crate libc;
extern crate rust_jvm;

use rust_jvm::vm::sig;
use rust_jvm::vm::value::Value;
use std::num::Wrapping;

pub mod math;
pub mod system;

#[no_mangle]
pub unsafe extern "C" fn Test_doubleIt(argc: usize, argv: *const Value) -> Option<Value> {
    match argc {
        1 => {
            if let Value::Reference(ref obj) = *argv.offset(0) {
                let field = sig::Field {
                    name: String::from("a"),
                    ty: sig::Type::Int,
                };
                let val = {
                    let test = obj.borrow();
                    test.get_field(&field)
                };
                if let Value::Int(val) = val {
                    obj.borrow_mut().put_field(field, Value::Int(val * Wrapping(2)));
                } else {
                    panic!("Not an int");
                }
            }
        }
        _ => (),
    }
    None
}
