use super::super::model;
use super::symref;
use super::ConstantPool;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Class {
    pub symref: symref::Class,
    pub access_flags: u16,
    pub superclass: Option<Box<Class>>,
    constant_pool: ConstantPool,
    methods: HashMap<String, Method>
}

impl Class {
    pub fn new(symref: symref::Class,
               superclass: Option<Box<Class>>,
               constant_pool: ConstantPool,
               class: model::class::Class) -> Self {
        let mut methods = HashMap::new();
        for method_info in class.methods.iter() {

        }
        Class {
            symref: symref,
            access_flags: 0,
            superclass: superclass,
            constant_pool: constant_pool,
            methods: methods
        }
    }
}

#[derive(Debug)]
pub struct Method {
    pub symref: symref::Method,
    pub access_flags: u16,
//    pub code: MethodCode
}

#[derive(Debug)]
struct MethodCode {
    max_locals: u16,
    code: Box<[u8]>
}

