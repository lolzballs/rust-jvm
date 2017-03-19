use super::class::Class;
use super::sig;

use std::cell::RefCell;
use std::collections::HashMap;
use std::num::Wrapping;
use std::rc::Rc;

#[derive(Clone, Debug)]
#[repr(C)]
pub enum Value {
    Int(Wrapping<i32>),
    Float(f32),
    Long(Wrapping<i64>),
    Double(f64),
    Reference(Rc<RefCell<Scalar>>),
    ArrayReference(Rc<RefCell<Array>>),
    NullReference,
}

#[derive(Debug)]
pub struct Scalar {
    class: Rc<Class>,
    fields: HashMap<sig::Field, Value>,
}

impl Scalar {
    pub fn new(class: Rc<Class>) -> Self {
        match class.symref.sig {
            sig::Class::Scalar(_) => {
                let field_sigs = class.collect_instance_fields();
                let mut fields = HashMap::new();
                // TODO: load instance fields
                for field in field_sigs {
                    let value = field.ty.get_default();
                    fields.insert(field, value);
                }
                Scalar {
                    class: class,
                    fields: fields,
                }
            }
            _ => panic!("Scalar value must be a scalar class"),
        }
    }

    pub fn put_field(&mut self, sig: sig::Field, value: Value) {
        self.fields.insert(sig, value);
    }

    pub fn get_field(&self, sig: &sig::Field) -> Value {
        self.fields[sig].clone()
    }
}

#[derive(Debug)]
pub struct Array {
    class: Rc<Class>,
    array: Vec<Value>,
}

impl Array {
    pub fn new(class: Rc<Class>, size: i32) -> Self {
        let mut array = Vec::with_capacity(size as usize);
        match class.symref.sig {
            sig::Class::Array(ref component) => {
                for _ in 0..size {
                    array.push(component.get_default());
                }
            }
            sig::Class::Scalar(_) => panic!("Array classes cannot be Scalar!"),
        }
        Array {
            class: class,
            array: array,
        }
    }

    pub fn copy_from(&mut self, other: Rc<RefCell<Array>>, src: i32, dst: i32, len: i32) {
        let other = other.borrow();
        self.array.truncate(dst as usize);
        self.array.extend(other.array[src as usize..(src + len) as usize].iter().cloned());
    }

    pub fn len(&self) -> i32 {
        self.array.len() as i32
    }

    pub fn get(&self, index: usize) -> Value {
        self.array[index].clone()
    }

    pub fn insert(&mut self, index: usize, value: Value) {
        self.array[index] = value;
    }
}
