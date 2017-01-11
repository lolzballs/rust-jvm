use super::ConstantPool;
use super::frame;
use super::super::model;
use super::sig;
use super::symref;
use super::value::Value;

use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Debug)]
pub struct Class {
    pub symref: symref::Class,
    pub access_flags: u16,
    pub superclass: Option<Box<Class>>,
    constant_pool: ConstantPool,
    methods: HashMap<sig::Method, Method>,
    field_constants: HashMap<sig::Field, u16>,
    field_values: RefCell<Option<HashMap<sig::Field, Value>>>,
}

impl Class {
    pub fn new(symref: symref::Class,
               superclass: Option<Box<Class>>,
               constant_pool: ConstantPool,
               class: model::class::Class)
               -> Self {
        let mut field_constants = HashMap::new();
        for field_info in class.fields.iter() {
            let name = constant_pool.lookup_utf8(field_info.name_index);
            let ty = sig::Type::new(constant_pool.lookup_utf8(field_info.descriptor_index))
                .unwrap();
            let sig = sig::Field::new(name.clone(), ty);
            // If the field is static, add to field_constants
            if field_info.access_flags & model::info::field::ACC_STATIC != 0 {
                for attr in field_info.attributes.iter() {
                    if let model::info::Attribute::ConstantValue { value_index } = *attr {
                        field_constants.insert(sig.clone(), value_index);
                    }
                }
            }
        }

        let mut methods = HashMap::new();
        for method_info in class.methods.iter() {
            let name = constant_pool.lookup_utf8(method_info.name_index);
            let descriptor = constant_pool.lookup_utf8(method_info.descriptor_index);
            let sig = sig::Method::new(name.clone(), descriptor.clone());
            let method = Method::new(symref::Method {
                                         class: symref.clone(),
                                         sig: sig.clone(),
                                     },
                                     method_info);

            methods.insert(sig, method);
        }
        Class {
            symref: symref,
            access_flags: 0,
            superclass: superclass,
            constant_pool: constant_pool,
            methods: methods,
            field_constants: field_constants,
            field_values: RefCell::new(None),
        }
    }

    pub fn initialize(&self) {
        // Initialize all the field_values
        let run_clinit = match *self.field_values.borrow() {
            None => true,
            Some(_) => false,
        };
        if run_clinit {
            let mut field_values = HashMap::new();
            for (sig, index) in &self.field_constants {
                let value = self.constant_pool.resolve_literal(*index);
                field_values.insert(sig.clone(), value.clone());
            }
            *self.field_values.borrow_mut() = Some(field_values);

            let clinit_sig = sig::Method {
                name: String::from("<clinit>"),
                params: vec![],
                return_type: None,
            };
            match self.methods.get(&clinit_sig) {
                None => (),
                Some(ref method) => {
                    let _ = method.invoke(&self, None);
                }
            }
        }
    }

    pub fn get_constant_pool(&self) -> &ConstantPool {
        &self.constant_pool
    }

    pub fn find_method(&self, method_symref: &symref::Method) -> &Method {
        self.initialize();
        self.methods
            .get(&method_symref.sig)
            .unwrap_or_else(|| {
                panic!("{:?} is not in this class({:?})",
                       method_symref.sig,
                       &self.symref.sig)
            })
    }

    pub fn get_field(&self, field_symref: &symref::Field) -> Value {
        self.initialize();
        let map_opt = self.field_values.borrow();
        let map = map_opt.as_ref().unwrap();
        map.get(&field_symref.sig).unwrap().clone()
        // TODO: Superclass stuff
    }

    pub fn put_field(&self, field_symref: &symref::Field, value: Value) {
        self.initialize();
        let mut map_opt = self.field_values.borrow_mut();
        let mut map = map_opt.as_mut().unwrap();
        map.insert(field_symref.sig.clone(), value);
    }
}

#[derive(Debug)]
pub struct Method {
    pub symref: symref::Method,
    pub access_flags: u16,
    code: MethodCode,
}

impl Method {
    pub fn new(symref: symref::Method, info: &model::info::Method) -> Self {
        let method_code = info.attributes
            .iter()
            .fold(None, |code, attr| {
                code.or(match *attr {
                    model::info::Attribute::Code { max_locals, ref code, .. } => {
                        Some(MethodCode {
                            max_locals: max_locals,
                            code: code.clone(),
                        })
                    }
                    _ => None,
                })
            })
            .unwrap();

        Method {
            symref: symref,
            access_flags: info.access_flags,
            code: method_code,
        }
    }

    pub fn invoke(&self, class: &Class, args_opt: Option<Vec<Value>>) -> Option<Value> {
        let max_locals = self.code.max_locals as usize;
        let mut locals = Vec::with_capacity(max_locals);
        match args_opt {
            Some(args) => {
                for value in args {
                    locals.push(Some(value));
                }
            }
            None => (),
        }
        while locals.len() < max_locals {
            locals.push(None);
        }
        let frame = frame::Frame::new(class, &*self.code.code, locals);
        frame.run()
    }
}

#[derive(Debug)]
struct MethodCode {
    max_locals: u16,
    code: Box<[u8]>,
}
