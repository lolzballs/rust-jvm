use std::cell::RefCell;
use std::ops::Index;
use std::num::Wrapping;
use std::rc::Rc;

use super::class_loader::ClassLoader;
use super::value::{Array, Scalar, Value};
use super::symref;
use super::sig;

use model::info::Constant;

#[derive(Debug)]
pub enum ConstantPoolEntry {
    Literal(Value),
    ClassRef(symref::Class),
    MethodRef(symref::Method),
    FieldRef(symref::Field),
    StringValue(String),
    UnresolvedString(u16),
}

#[derive(Debug)]
pub struct ConstantPool {
    entries: Vec<Option<ConstantPoolEntry>>,
}

impl Index<u16> for ConstantPool {
    type Output = Option<ConstantPoolEntry>;

    fn index(&self, index: u16) -> &Self::Output {
        &self.entries[(index - 1) as usize]
    }
}

impl ConstantPool {
    pub fn new(constant_pool: &Box<[Constant]>) -> Self {
        let mut entries = vec![];
        for constant in constant_pool.iter() {
            let entry = match *constant {
                Constant::Class { .. } => {
                    let symref = Self::force_class_ref(constant_pool, constant);
                    Some(ConstantPoolEntry::ClassRef(symref))
                }
                Constant::Methodref { .. } => {
                    let symref = Self::force_method_ref(constant_pool, constant);
                    Some(ConstantPoolEntry::MethodRef(symref))
                }
                Constant::Fieldref { .. } => {
                    let symref = Self::force_field_ref(constant_pool, constant);
                    Some(ConstantPoolEntry::FieldRef(symref))
                }
                Constant::Integer { value } => {
                    Some(ConstantPoolEntry::Literal(Value::Int(Wrapping(value))))
                }
                Constant::Long { value } => {
                    Some(ConstantPoolEntry::Literal(Value::Long(Wrapping(value))))
                }
                Constant::Float { value } => Some(ConstantPoolEntry::Literal(Value::Float(value))),
                Constant::Double { value } => {
                    Some(ConstantPoolEntry::Literal(Value::Double(value)))
                }
                Constant::NameAndType { .. } |
                Constant::Nothing => None,
                Constant::Utf8 { .. } => {
                    let symref = Self::force_string(constant);
                    Some(ConstantPoolEntry::StringValue(symref.clone()))
                }
                Constant::String { string_index } => {
                    Some(ConstantPoolEntry::UnresolvedString(string_index))
                }
                _ => {
                    panic!("Unimplemented constant: {:#?}", constant);
                }
            };
            entries.push(entry);
        }
        ConstantPool { entries: entries }
    }

    fn force_class_ref(constant_pool: &Box<[Constant]>, info: &Constant) -> symref::Class {
        match *info {
            Constant::Class { name_index } => {
                let name = Self::force_string(&constant_pool[(name_index - 1) as usize]);
                symref::Class { sig: sig::Class::new(name) }
            }
            _ => {
                panic!("Constant {:?} must be Constant::Class", info);
            }
        }
    }

    fn force_method_ref(constant_pool: &Box<[Constant]>, info: &Constant) -> symref::Method {
        match *info {
            Constant::Methodref { class_index, name_and_type_index } => {
                let class = Self::force_class_ref(constant_pool,
                                                  &constant_pool[(class_index - 1) as usize]);
                let (name, descriptor) =
                    Self::force_name_and_type(constant_pool,
                                              &constant_pool[(name_and_type_index - 1) as usize]);
                symref::Method {
                    class: class,
                    sig: sig::Method::new(name.clone(), descriptor.clone()),
                }
            }
            _ => {
                panic!("Constant {:?} must be Constant::Class", info);
            }
        }
    }

    fn force_field_ref(constant_pool: &Box<[Constant]>, info: &Constant) -> symref::Field {
        match *info {
            Constant::Fieldref { class_index, name_and_type_index } => {
                let class = Self::force_class_ref(constant_pool,
                                                  &constant_pool[(class_index - 1) as usize]);
                let (name, descriptor) =
                    Self::force_name_and_type(constant_pool,
                                              &constant_pool[(name_and_type_index - 1) as usize]);

                let (name, descriptor) = (name.clone(), descriptor.clone());
                let ty = sig::Type::new(&descriptor);
                symref::Field {
                    class: class,
                    sig: sig::Field::new(name, ty.unwrap()),
                }
            }
            _ => {
                panic!("Constant {:?} must be Constant::Class", info);
            }
        }
    }

    fn force_string(info: &Constant) -> &String {
        match *info {
            Constant::Utf8 { length, ref value } => value,
            _ => {
                panic!("Constant {:?} must be Constant::Utf8", info);
            }
        }
    }

    fn force_name_and_type<'a>(constant_pool: &'a Box<[Constant]>,
                               info: &'a Constant)
                               -> (&'a String, &'a String) {
        match *info {
            Constant::NameAndType { name_index, descriptor_index } => {
                let name = Self::force_string(&constant_pool[(name_index - 1) as usize]);
                let descriptor =
                    Self::force_string(&constant_pool[(descriptor_index - 1) as usize]);
                (name, descriptor)
            }
            _ => {
                panic!("Constant {:?} must be Constant::NameAndType", info);
            }
        }
    }

    pub fn resolve_literal(&self, index: u16, class_loader: &mut ClassLoader) -> Value {
        match self.entries[(index - 1) as usize] {
            Some(ConstantPoolEntry::Literal(ref value)) => value.clone(),
            Some(ConstantPoolEntry::UnresolvedString(value)) => {
                let array_sig = sig::Class::Array(Box::new(sig::Type::Char));
                let array_class = class_loader.resolve_class(&array_sig);

                let chars = {
                    if let Some(ConstantPoolEntry::StringValue(ref string)) =
                        self.entries[(value - 1) as usize] {
                        string.clone().into_bytes()
                    } else {
                        panic!("UnresolvedString {} must point to a StringValue", value);
                    }
                };

                // CONVERT TO UTF-8
                let mut array = Array::new(array_class, chars.len() as i32);
                for (i, c) in chars.iter().enumerate() {
                    array.insert(i, Value::Int(Wrapping(*c as i32)));
                }
                let array_rc = Rc::new(RefCell::new(array));

                let string_sig = sig::Class::Scalar(String::from("java/lang/String"));
                let string_symref = symref::Class { sig: string_sig.clone() };
                let string_class = class_loader.resolve_class(&string_sig);
                let string = Scalar::new(string_class.clone());
                let string_rc = Rc::new(RefCell::new(string));

                let constructor_sig = sig::Method {
                    name: String::from("<init>"),
                    params: vec![sig::Type::Reference(array_sig.clone())],
                    return_type: None,
                };
                let constructor_symref = symref::Method {
                    class: string_symref,
                    sig: constructor_sig,
                };
                let constructor = string_class.find_method(class_loader, &constructor_symref);
                let args = Some(vec![Value::Reference(string_rc.clone()),
                                     Value::ArrayReference(array_rc)]);
                let result = constructor.borrow().invoke(string_class.as_ref(), class_loader, args);
                match result {
                    None => (),
                    Some(_) => panic!("<init> returned a value"),
                }
                Value::Reference(string_rc)
            }
            ref value => {
                panic!("Item at index {} must be ConstantPoolEntry::Literal found {:?}",
                       index,
                       value);
            }
        }
    }

    pub fn lookup_utf8(&self, index: u16) -> &String {
        match self.entries[(index - 1) as usize] {
            Some(ConstantPoolEntry::StringValue(ref string)) => string,
            _ => {
                panic!("Item at index {} must be ConstantPoolEntry::StringValue",
                       index);
            }
        }
    }
}
