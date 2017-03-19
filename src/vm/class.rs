use super::ClassLoader;
use super::ConstantPool;
use super::frame;
use super::super::model;
use super::native;
use super::sig;
use super::symref;
use super::value::Value;

use lib::Library;

use std::collections::{HashMap, HashSet};
use std::cell::RefCell;
use std::rc::Rc;

/// A representation of a loaded class.
#[derive(Debug)]
pub struct Class {
    /// A symbolic reference to the class.
    pub symref: symref::Class,
    /// The access flags of this class.
    pub access_flags: u16,
    /// The super class of the class.
    /// If it is `java/lang/Object`, this is `None`.
    pub superclass: Option<Box<Class>>,
    constant_pool: ConstantPool,
    methods: HashMap<sig::Method, RefCell<Method>>,
    fields: HashMap<sig::Field, u16>,
    field_constants: HashMap<sig::Field, u16>,
    field_values: RefCell<Option<HashMap<sig::Field, Value>>>,
}

impl Class {
    /// Creates a new `Class` from its symbolic reference,
    /// superclass, runtime constant pool, and class model.
    ///
    /// Returns the new class, as well as a vector of unbound native methods.
    pub fn new(symref: symref::Class,
               superclass: Option<Box<Class>>,
               constant_pool: ConstantPool,
               class: model::class::Class)
               -> (Self, Vec<sig::Method>) {
        let mut fields = HashMap::new();
        let mut field_constants = HashMap::new();
        for field_info in class.fields.iter() {
            let name = constant_pool.lookup_utf8(field_info.name_index);
            let ty = sig::Type::new(constant_pool.lookup_utf8(field_info.descriptor_index))
                .unwrap();
            let sig = sig::Field::new(name.clone(), ty);
            fields.insert(sig.clone(), field_info.access_flags);
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
        let mut unbound_natives = Vec::new();
        for method_info in class.methods.iter() {
            let name = constant_pool.lookup_utf8(method_info.name_index);
            let descriptor = constant_pool.lookup_utf8(method_info.descriptor_index);
            let sig = sig::Method::new(name.clone(), descriptor.clone());

            let method = Method::new(symref::Method {
                                         class: symref.clone(),
                                         sig: sig.clone(),
                                     },
                                     method_info);

            methods.insert(sig.clone(), RefCell::new(method));

            if method_info.access_flags & model::info::method::ACC_NATIVE != 0 {
                unbound_natives.push(sig);
            }
        }
        (Class {
             symref: symref,
             access_flags: 0,
             superclass: superclass,
             constant_pool: constant_pool,
             methods: methods,
             fields: fields,
             field_constants: field_constants,
             field_values: RefCell::new(None),
         },
         unbound_natives)
    }

    /// Creates a new `Class` representing an array of type `component`.
    pub fn new_array(component: sig::Type) -> Self {
        // TODO: Length field, access flags
        let sig = sig::Class::Array(Box::new(component));
        let symref = symref::Class { sig: sig };

        let constant_pool: Vec<model::info::Constant> = Vec::new();
        // TODO: Optimize this
        Class {
            symref: symref,
            access_flags: 0,
            superclass: None,
            constant_pool: ConstantPool::new(&constant_pool.into_boxed_slice()),
            methods: HashMap::new(),
            fields: HashMap::new(),
            field_constants: HashMap::new(),
            field_values: RefCell::new(None),
        }
    }

    /// Initialize a `Class`, calling its `<clinit>` if necessary.
    pub fn initialize(&self, class_loader: &mut ClassLoader) {
        // Initialize all the field_values
        let run_clinit = match *self.field_values.borrow() {
            None => true,
            Some(_) => false,
        };
        if run_clinit {
            let mut field_values = HashMap::new();
            for (sig, index) in &self.field_constants {
                let value = self.constant_pool.resolve_literal(*index, class_loader);
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
                    let _ = method.borrow().invoke(&self, class_loader, None);
                }
            }
        }
    }

    /// Bind `sig` to `library`.
    ///
    /// # Panics
    /// Panics if `sig` isn't in `methods`, or if `sig` isn't a Native method.
    pub fn bind_native_method(&self, sig: sig::Method, library: Rc<Library>) {
        let mut method = self.methods.get(&sig).unwrap().borrow_mut();
        method.bind_native(library);
    }

    /// Returns a reference to the runtime constant pool of this class.
    pub fn get_constant_pool(&self) -> &ConstantPool {
        &self.constant_pool
    }

    /// Returns a set of all non-static fields of this class.
    pub fn collect_instance_fields(&self) -> HashSet<sig::Field> {
        // TODO: Superclass fields
        let mut fields = HashSet::new();
        for (sig, access_flags) in &self.fields {
            if access_flags & model::info::field::ACC_STATIC == 0 {
                fields.insert(sig.clone());
            }
        }
        fields
    }

    /// Returns a reference to a `Method`, initializing the class if it hasn't been already.
    ///
    /// # Panics
    /// Panics if `method_symref` isn't in `methods`.
    pub fn find_method(&self,
                       class_loader: &mut ClassLoader,
                       method_symref: &symref::Method)
                       -> &RefCell<Method> {
        self.initialize(class_loader);
        self.methods
            .get(&method_symref.sig)
            .unwrap_or_else(|| {
                panic!("{:?} is not in this class({:?})",
                       method_symref.sig,
                       &self.symref.sig)
            })
    }

    /// Returns the `Value` of a field, initializing the class if it hasn't been already.
    ///
    /// # Panics
    /// Panics if the field_symref isn't in `field_values`.
    pub fn get_field(&self, class_loader: &mut ClassLoader, field_symref: &symref::Field) -> Value {
        self.initialize(class_loader);
        let map_opt = self.field_values.borrow();
        let map = map_opt.as_ref().unwrap();
        map.get(&field_symref.sig).unwrap().clone()
        // TODO: Superclass stuff
    }

    /// Sets a field, initializing the class if it hasn't been already.
    pub fn put_field(&self,
                     class_loader: &mut ClassLoader,
                     field_symref: &symref::Field,
                     value: Value) {
        self.initialize(class_loader);
        let mut map_opt = self.field_values.borrow_mut();
        let mut map = map_opt.as_mut().unwrap();
        map.insert(field_symref.sig.clone(), value);
    }
}

/// A representation of a loaded method.
#[derive(Debug)]
pub struct Method {
    /// A symbolic reference to the method.
    pub symref: symref::Method,
    /// The access flags of the method.
    pub access_flags: u16,
    code: MethodCode,
}

impl Method {
    /// Creates a new `Method`.
    pub fn new(symref: symref::Method, info: &model::info::Method) -> Self {
        let method_code = {
            if info.access_flags & model::info::method::ACC_NATIVE != 0 {
                MethodCode::UnresolvedNative
            } else {
                info.attributes
                    .iter()
                    .fold(None, |code, attr| {
                        code.or(match *attr {
                            model::info::Attribute::Code { max_locals, ref code, .. } => {
                                Some(MethodCode::Java {
                                    max_locals: max_locals,
                                    code: code.clone(),
                                })
                            }
                            _ => None,
                        })
                    })
                    .unwrap()
            }
        };
        Method {
            symref: symref,
            access_flags: info.access_flags,
            code: method_code,
        }
    }

    /// Bind to a native library.
    ///
    /// # Panics
    /// Panics if the method is not a native, or is already bound.
    pub fn bind_native(&mut self, lib: Rc<Library>) {
        if let MethodCode::UnresolvedNative = self.code {
            self.code = MethodCode::Native(lib);
        } else {
            panic!("Cannot bind a non-native method to a native method")
        }
    }

    /// Invoke the method.
    ///
    /// If the method is a Native method, invokes the native method.
    /// If the method is a Java method, creates a new frame and invokes it.
    ///
    /// # Panics
    /// Panics if the method is a Native method and is Unresolved.
    pub fn invoke(&self,
                  class: &Class,
                  class_loader: &mut ClassLoader,
                  args_opt: Option<Vec<Value>>)
                  -> Option<Value> {
        match self.code {
            MethodCode::Native(ref lib) => native::invoke(&lib.clone(), &self.symref, args_opt),
            MethodCode::UnresolvedNative => panic!("{:?} native not loaded!", self.symref.sig),
            MethodCode::Java { max_locals, ref code } => {
                let max_locals = max_locals as usize;
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
                let frame = frame::Frame::new(class, &*code, locals);
                frame.run(class_loader)
            }
        }
    }
}

/// Represents the Method's code
#[derive(Debug)]
enum MethodCode {
    /// A bound native method.
    Native(Rc<Library>),
    /// An unbound native method.
    UnresolvedNative,
    /// A java method
    Java { max_locals: u16, code: Box<[u8]> },
}
