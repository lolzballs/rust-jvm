use vm::value::Value;
use vm::symref;

use lib::{Library, Symbol};

use std::io::Result;
use std::ptr;

pub type JniFn = unsafe fn(usize, *const Value) -> Option<Value>;

pub fn load(path: &str) -> Library {
    Library::new(path).unwrap()
}

pub fn has_method(lib: &Library, symref: &symref::Method) -> bool {
    unsafe { lib.get::<JniFn>(b"java_lang_System_println\0").is_ok() }
}

pub fn invoke(lib: &Library,
              symref: &symref::Method,
              args_opt: Option<Vec<Value>>)
              -> Option<Value> {
    unsafe {
        let func: Symbol<JniFn> = lib.get(b"java_lang_System_println\0").unwrap();
        if args_opt.is_none() {
            func(0, ptr::null())
        } else {
            let args = args_opt.unwrap();
            func(args.len(), args.as_ptr())
        }
    }
}
