#![no_std]

extern crate libc;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum Value {
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    NullReference,
}

unsafe fn println1(i: Value) {
    if let Value::Int(i) = i {
        libc::printf((b"%d\n\0").as_ptr() as *const libc::c_char, i);
    }
}

unsafe fn println2(a: Value, b: Value) {
    if let Value::Int(a) = a {
        if let Value::Int(b) = b {
            libc::printf((b"%d %d\n\0").as_ptr() as *const libc::c_char, a, b);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn java_lang_System_println(argc: usize,
                                                  argv: *const Value)
                                                  -> Option<Value> {
    match argc {
        1 => println1(*argv.offset(0)),
        2 => println2(*argv.offset(0), *argv.offset(1)),
        _ => (),
    }

    None
}
