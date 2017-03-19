use super::symref;
use super::sig;
use super::value::Value;

use lib::{Library, Symbol};

use std::ffi::CString;
use std::ptr;

pub type JniFn = unsafe fn(usize, *const Value) -> Option<Value>;

pub fn load(path: &str) -> Library {
    Library::new(path).unwrap()
}

fn get_function_signature(symref: &symref::Method) -> CString {
    let mut signature = String::new();
    if let sig::Class::Scalar(ref class_sig) = symref.class.sig {
        for class_segment in class_sig.split('/') {
            signature.push_str(class_segment);
            signature.push_str("_");
        }
    } else {
        panic!("Error invoking {:?}, only Scalar classes can have native methods",
               symref);
    }

    signature.push_str(symref.sig.name.replace("_", "1_").as_str());

    CString::new(signature).unwrap()
}

pub fn has_method(lib: &Library, symref: &symref::Method) -> bool {
    unsafe { lib.get::<JniFn>(get_function_signature(symref).as_bytes()).is_ok() }
}

pub fn invoke(lib: &Library,
              symref: &symref::Method,
              args_opt: Option<Vec<Value>>)
              -> Option<Value> {

    unsafe {
        let func: Symbol<JniFn> = lib.get(get_function_signature(symref).as_bytes()).unwrap();
        if args_opt.is_none() {
            func(0, ptr::null())
        } else {
            let args = args_opt.unwrap();
            func(args.len(), args.as_ptr())
        }
    }
}
