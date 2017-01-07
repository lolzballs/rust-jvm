use super::super::model;
use super::sig;
use super::symref;
use super::ConstantPool;
use super::Value;

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
    field_values: RefCell<Option<HashMap<sig::Field, Value>>>
}

impl Class {
    pub fn new(symref: symref::Class,
               superclass: Option<Box<Class>>,
               constant_pool: ConstantPool,
               class: model::class::Class) -> Self {
        let mut field_constants = HashMap::new();
        for field_info in class.fields.iter() {
            let name = constant_pool.lookup_utf8(field_info.name_index);
            let ty = sig::Type::new(
                constant_pool.lookup_utf8(field_info.descriptor_index)).unwrap();
            let sig = sig::Field::new(name.clone(), ty);
            if field_info.access_flags & model::info::field::ACC_STATIC != 0 {
                for attr in field_info.attributes.iter() {
                    if let model::info::Attribute::ConstantValue { value_index }
                    = *attr {
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
            let method = Method::new(
                symref::Method {
                    class: symref.clone(),
                    sig: sig.clone()
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
            field_values: RefCell::new(None)
        }
    }

    pub fn initialize(&self) {
        let mut field_values = HashMap::new();
        for (sig, index) in &self.field_constants {
            let value = self.constant_pool.resolve_literal(*index);
            field_values.insert(sig.clone(), value.clone());
        }

        *self.field_values.borrow_mut() = Some(field_values);
    }
}

#[derive(Debug)]
pub struct Method {
    pub symref: symref::Method,
    pub access_flags: u16,
    code: MethodCode
}

impl Method {
    pub fn new(symref: symref::Method, info: &model::info::Method) -> Self {
        let method_code = info.attributes.iter().fold(None, |code, attr| {
            code.or(
                match *attr {
                    model::info::Attribute::Code { max_locals, ref code, .. } => {
                        Some(MethodCode {
                            max_locals: max_locals,
                            code: code.clone()
                        })
                    },
                    _ => None
                }
                )
            }).unwrap();

        Method {
            symref: symref,
            access_flags: info.access_flags,
            code: method_code
        }
    }
}

#[derive(Debug)]
struct MethodCode {
    max_locals: u16,
    code: Box<[u8]>
}

