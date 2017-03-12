use rust_jvm::vm::value::Value;

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
