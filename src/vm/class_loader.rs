use super::class;
use super::super::model;
use super::sig;
use super::symref;
use super::constant_pool::{ConstantPool, ConstantPoolEntry};

use std::collections::HashMap;
use std::io::Read;
use std::fs::File;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug)]
pub struct ClassLoader {
    runtime_path: PathBuf,
    classes: HashMap<sig::Class, Rc<class::Class>>,
}

impl ClassLoader {
    pub fn new(runtime_path: PathBuf) -> ClassLoader {
        ClassLoader {
            runtime_path: runtime_path,
            classes: HashMap::new(),
        }
    }

    fn find_class_bytes(&self, name: &str) -> Result<Vec<u8>, &'static str> {
        let path = PathBuf::from(String::from(name) + ".class");
        if path.exists() {
            File::open(path)
                .and_then(|mut file| {
                    let mut buf = Vec::new();
                    try!(file.read_to_end(&mut buf));
                    Ok(buf)
                })
                .or(Err("Could not load class"))
        } else {
            let rt_path = self.runtime_path.join(String::from(name) + ".class");
            if rt_path.exists() {
                File::open(rt_path)
                    .and_then(|mut file| {
                        let mut buf = Vec::new();
                        try!(file.read_to_end(&mut buf));
                        Ok(buf)
                    })
                    .or(Err("Could not load class"))
            } else {
                Err("Class not found")
            }
        }
    }

    fn load_class_bytes(&mut self, sig: &sig::Class, bytes: Vec<u8>) -> Rc<class::Class> {
        let class = model::Class::new(bytes);
        self.load_class(sig, class)
    }

    fn load_class(&mut self, sig: &sig::Class, model: model::Class) -> Rc<class::Class> {
        let rcp = ConstantPool::new(&model.constant_pool);
        let sigs_match = {
            if let Some(ConstantPoolEntry::ClassRef(ref symref)) = rcp[model.this_class] {
                *sig == symref.sig
            } else {
                panic!("this_class({}) must point to a ClassRef", model.this_class);
            }
        };
        if sigs_match {
            let symref = symref::Class { sig: sig.clone() };
            let class = class::Class::new(symref, None, rcp, model);
            let rc = Rc::new(class);
            self.classes.insert(sig.clone(), rc.clone());
            rc
        } else {
            panic!("Class signature mismatch: given {:?}", sig);
        }
    }

    pub fn resolve_class(&mut self, sig: &sig::Class) -> Rc<class::Class> {
        if let Some(class) = self.classes.get(&sig) {
            // the class has been resolved
            return class.clone();
        }

        match *sig {
            sig::Class::Scalar(ref name) => {
                let class_bytes = self.find_class_bytes(name).unwrap();
                self.load_class_bytes(sig, class_bytes)
            }
            sig::Class::Array(ref component) => {
                Rc::new(class::Class::new_array(*component.clone()))
            }
        }
    }
}
