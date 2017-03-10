extern crate libc;
extern crate rust_jvm;

use rust_jvm::vm::sig;
use rust_jvm::vm::value::Value;
use std::num::Wrapping;

unsafe fn println1(i: &Value) {
    if let Value::Int(i) = *i {
        libc::printf((b"%d\n\0").as_ptr() as *const libc::c_char, i);
    }
}

unsafe fn println2(a: &Value, b: &Value) {
    if let Value::Int(a) = *a {
        if let Value::Int(b) = *b {
            libc::printf((b"%d %d\n\0").as_ptr() as *const libc::c_char, a, b);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn java_lang_System_println(argc: usize,
                                                  argv: *const Value)
                                                  -> Option<Value> {
    match argc {
        1 => {
            let arg = &*argv.offset(0);
            println1(arg)
        }
        2 => {
            let arg1 = &*argv.offset(0);
            let arg2 = &*argv.offset(1);
            println2(arg1, arg2)
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
                if let Value::Int(Wrapping(val)) = val {
                    obj.borrow_mut().put_field(field, Value::Int(Wrapping(val * 2)));
                } else {
                }
            }
        }
        _ => (),
    }
    None
}
