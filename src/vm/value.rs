use super::class::Class;
use super::sig;

use std::cell::RefCell;
use std::collections::HashMap;
use std::num::Wrapping;
use std::rc::Rc;

#[derive(Clone, Debug)]
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
        let fields = HashMap::new();
        // TODO: load instance fields
        Scalar {
            class: class,
            fields: fields,
        }
    }
}

#[derive(Debug)]
pub struct Array {
    component: sig::Type,
    array: Vec<Value>,
}

impl Array {
    pub fn new(component: sig::Type, size: i32) -> Self {
        let mut array = Vec::with_capacity(size as usize);
        for _ in 0..size {
            array.push(component.get_default());
        }
        Array {
            component: component,
            array: array,
        }
    }

    pub fn get(&self, index: usize) -> Value {
        self.array[index].clone()
    }

    pub fn insert(&mut self, index: usize, value: Value) {
        self.array[index] = value;
    }
}
