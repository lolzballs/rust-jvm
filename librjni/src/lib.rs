extern crate libc;
extern crate rust_jvm;

use rust_jvm::vm::sig;
use rust_jvm::vm::value::Value;
use std::num::Wrapping;

#[no_mangle]
pub unsafe extern "C" fn java_lang_System_write(argc: usize, argv: *const Value) -> Option<Value> {
    match argc {
        1 => {
            let arg = &*argv.offset(0);
            match *arg {
                Value::Int(ref i) => {
                    libc::putchar(i.0);
                }
                _ => (),
            }
        }
        _ => (),
    }

    None
}

#[no_mangle]
pub unsafe extern "C" fn java_lang_System_readInt(argc: usize,
                                                  argv: *const Value)
                                                  -> Option<Value> {
    match argc {
        0 => {
            let d: i32 = -1;
            libc::scanf(b"%d\0".as_ptr() as *const libc::c_char, &d);
            Some(Value::Int(Wrapping(d)))
        }
        _ => None,
    }
}

#[no_mangle]
pub unsafe extern "C" fn java_lang_System_arraycopy(argc: usize,
                                                    argv: *const Value)
                                                    -> Option<Value> {
    match argc {
        5 => {
            let src = {
                if let Value::ArrayReference(ref array) = *argv.offset(0) {
                    array.clone()
                } else {
                    return None;
                }
            };
            let srcPos = {
                if let Value::Int(value) = *argv.offset(1) {
                    value
                } else {
                    return None;
                }
            };
            let dst = {
                if let Value::ArrayReference(ref array) = *argv.offset(2) {
                    array.clone()
                } else {
                    return None;
                }
            };
            let dstPos = {
                if let Value::Int(value) = *argv.offset(3) {
                    value
                } else {
                    return None;
                }
            };
            let count = {
                if let Value::Int(value) = *argv.offset(4) {
                    value
                } else {
                    return None;
                }
            };

            dst.borrow_mut().copy_from(src, srcPos.0, dstPos.0, count.0);
        }
        _ => (),
    }
    None
}

#[no_mangle]
pub unsafe extern "C" fn java_lang_Math_log10(argc: usize, argv: *const Value) -> Option<Value> {
    match argc {
        1 => {
            match *argv {
                Value::Double(value) => Some(Value::Double(value.log10())),
                _ => None,
            }
        }
        _ => None,
    }
}

#[no_mangle]
pub unsafe extern "C" fn java_lang_Math_pow(argc: usize, argv: *const Value) -> Option<Value> {
    match argc {
        2 => {
            let a = match *argv {
                Value::Double(value) => value,
                _ => 0.0,
            };
            let b = match *argv.offset(1) {
                Value::Double(value) => value,
                _ => 0.0,
            };

            Some(Value::Double(a.powf(b)))
        }
        _ => None,
    }
}


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
