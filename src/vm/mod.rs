pub mod constant_pool;
pub mod runtime;
pub mod sig;
pub mod class;

pub mod symref {
    use super::sig;

    #[derive(Clone, Debug)]
    pub struct Class {
        pub sig: sig::Class
    }

    #[derive(Clone, Debug)]
    pub struct Method {
        pub class: Class,
        pub sig: sig::Method
    }

    #[derive(Clone, Debug)]
    pub struct Field {
        pub class: Class,
        pub sig: sig::Field
    }
}

pub use self::constant_pool::ConstantPool;

#[derive(Debug)]
pub enum Value {
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Reference(Box<class::Class>)
}

