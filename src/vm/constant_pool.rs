use super::Value;
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
    UnresolvedString(u16)
}

#[derive(Debug)]
pub struct ConstantPool {
    entries: Vec<Option<ConstantPoolEntry>>
}

impl ConstantPool {
    pub fn new(constant_pool: &Box<[Constant]>) -> Self {
        let mut entries = vec![];
        for constant in constant_pool.iter() {
            let entry = match *constant {
                Constant::Class { .. } => {
                    let symref = Self::force_class_ref(constant_pool, constant);
                    Some(ConstantPoolEntry::ClassRef(symref))
                },
                Constant::Methodref { .. } => {
                    let symref = Self::force_method_ref(constant_pool, constant);
                    Some(ConstantPoolEntry::MethodRef(symref))
                },
                Constant::Fieldref { .. } => {
                    let symref = Self::force_field_ref(constant_pool, constant);
                    Some(ConstantPoolEntry::FieldRef(symref))
                },
                Constant::Integer { value } => {
                    Some(ConstantPoolEntry::Literal(Value::Int(value)))
                },
                Constant::Long { value } => {
                    Some(ConstantPoolEntry::Literal(Value::Long(value)))
                },
                Constant::NameAndType { .. } => None,
                Constant::Utf8 { .. } => {
                    let symref = Self::force_string(constant);
                    Some(ConstantPoolEntry::StringValue(symref.clone()))
                },
                Constant::String { string_index } => {
                    Some(ConstantPoolEntry::UnresolvedString(string_index))
                },
                Constant::Nothing => {
                    None
                }
                _ => {
                    panic!("Unimplemented constant: {:#?}", constant);
                }
            };
            entries.push(entry);
        };
        ConstantPool {
            entries: entries
        }
    }

    fn force_class_ref(constant_pool: &Box<[Constant]>,
                       info: &Constant) -> symref::Class {
        match *info {
            Constant::Class { name_index } => {
                let name = Self::force_string(&constant_pool[(name_index - 1) as usize]);
                symref::Class {
                    sig: sig::Class::new(&name)
                }
            },
            _ => {
                panic!("Constant {:?} must be Constant::Class", info);
            }
        }
    }

    fn force_method_ref(constant_pool: &Box<[Constant]>,
                       info: &Constant) -> symref::Method {
        match *info {
            Constant::Methodref { class_index, name_and_type_index } => {
                let class = Self::force_class_ref(&constant_pool,
                                                  &constant_pool
                                                  [(class_index - 1) as usize]);
                let (name, descriptor)
                    = Self::force_name_and_type(&constant_pool,
                                                &constant_pool
                                                [(name_and_type_index - 1) as usize]);
                symref::Method {
                    class: class,
                    sig: sig::Method::new(name.clone(), descriptor.clone())
                }
            },
            _ => {
                panic!("Constant {:?} must be Constant::Class", info);
            }
        }
    }

    fn force_field_ref(constant_pool: &Box<[Constant]>,
                       info: &Constant) -> symref::Field {
        match *info {
            Constant::Fieldref { class_index, name_and_type_index } => {
                let class = Self::force_class_ref(&constant_pool,
                                                  &constant_pool
                                                  [(class_index - 1) as usize]);
                let (name, descriptor)
                    = Self::force_name_and_type(&constant_pool,
                                                &constant_pool
                                                [(name_and_type_index - 1) as usize]);

                let (name, descriptor) = (name.clone(), descriptor.clone());
                let ty = sig::Type::new(&descriptor);
                symref::Field {
                    class: class,
                    sig: sig::Field::new(name, ty.unwrap())
                }
            },
            _ => {
                panic!("Constant {:?} must be Constant::Class", info);
            }
        }
    }

    fn force_string(info: &Constant) -> &String {
        match *info {
            Constant::Utf8 { length, ref value } => {
                value
            },
            _ => {
                panic!("Constant {:?} must be Constant::Utf8", info);
            }
        }
    }

    fn force_name_and_type<'a>(constant_pool: &'a Box<[Constant]>,
                           info: &'a Constant) -> (&'a String, &'a String) {
        match *info {
            Constant::NameAndType { name_index, descriptor_index } => {
                let name = Self::force_string(&constant_pool[(name_index - 1) as usize]);
                let descriptor = Self::force_string(&constant_pool
                                                    [(descriptor_index - 1) as usize]);
                (name, descriptor)
            },
            _ => {
                panic!("Constant {:?} must be Constant::NameAndType", info);
            }
        }
    }

    pub fn resolve_literal(&self, index: u16) -> &Value {
        match self.entries[(index - 1) as usize] {
            Some(ConstantPoolEntry::Literal(ref value)) => value,
            _ => {
                panic!("Item at index {} must be ConstantPoolEntry::Literal", index);
            }
        }
    }

    pub fn lookup_utf8(&self, index: u16) -> &String {
        match self.entries[(index - 1) as usize] {
            Some(ConstantPoolEntry::StringValue(ref string)) => string,
            _ => {
                panic!("Item at index {} must be ConstantPoolEntry::StringValue", index);
            }
        }
    }
}

