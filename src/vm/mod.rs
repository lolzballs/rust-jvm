pub mod class;
pub mod constant_pool;
pub mod frame;
#[allow(dead_code)]
pub mod opcode;
pub mod sig;
pub mod symref;

pub use self::constant_pool::ConstantPool;

use std::cell::RefCell;
use std::num::Wrapping;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Value {
    Int(Wrapping<i32>),
    Float(f32),
    Long(Wrapping<i64>),
    Double(f64),
    Reference(Rc<RefCell<class::Class>>),
}
