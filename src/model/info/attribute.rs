use std::io::{Cursor, Read};

use super::Constant;

use byteorder::{BigEndian, ReadBytesExt};

#[derive(Debug)]
pub struct ExceptionHandler {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

#[derive(Debug)]
pub enum Attribute {
    ConstantValue { value_index: u16 },
    Code {
        length: u32,
        max_stack: u16,
        max_locals: u16,
        code_length: u32,
        code: Box<[u8]>,
        exception_table_length: u16,
        exception_table: Box<[ExceptionHandler]>,
        attributes_count: u16,
        attributes: Box<[Attribute]>,
    },
    Unknown {
        name_index: u16,
        length: u32,
        info: Box<[u8]>,
    },
}

impl Attribute {
    pub fn new(constant_pool: &Box<[Constant]>, cur: &mut Cursor<Vec<u8>>) -> Attribute {
        let name_index = cur.read_u16::<BigEndian>().unwrap() - 1; // 1-indexed
        let length = cur.read_u32::<BigEndian>().unwrap();

        let name = match constant_pool[name_index as usize] {
            Constant::Utf8 { length, ref value } => value,
            _ => {
                panic!("Attribute name_index({}) must point to Utf8", name_index);
            }
        };

        match name.as_ref() {
            "ConstantValue" => {
                Attribute::ConstantValue { value_index: cur.read_u16::<BigEndian>().unwrap() }
            }
            "Code" => {
                let max_stack = cur.read_u16::<BigEndian>().unwrap();
                let max_locals = cur.read_u16::<BigEndian>().unwrap();

                let code_length = cur.read_u32::<BigEndian>().unwrap();
                let code = vec![0; code_length as usize];
                let mut slice = code.into_boxed_slice();
                cur.read_exact(&mut slice).unwrap();

                let exception_table_length = cur.read_u16::<BigEndian>().unwrap();
                let mut exception_table = Vec::with_capacity(exception_table_length as usize);
                for _ in 0..exception_table_length {
                    exception_table.push(ExceptionHandler {
                        start_pc: cur.read_u16::<BigEndian>().unwrap(),
                        end_pc: cur.read_u16::<BigEndian>().unwrap(),
                        handler_pc: cur.read_u16::<BigEndian>().unwrap(),
                        catch_type: cur.read_u16::<BigEndian>().unwrap(),
                    });
                }

                let attributes_count = cur.read_u16::<BigEndian>().unwrap();
                let mut attributes = Vec::with_capacity(attributes_count as usize);
                for _ in 0..attributes_count {
                    attributes.push(Attribute::new(constant_pool, cur));
                }
                Attribute::Code {
                    length: length,
                    max_stack: max_stack,
                    max_locals: max_locals,
                    code_length: code_length,
                    code: slice,
                    exception_table_length: exception_table_length,
                    exception_table: exception_table.into_boxed_slice(),
                    attributes_count: attributes_count,
                    attributes: attributes.into_boxed_slice(),
                }
            }
            _ => {
                println!("Unknown attribute {}", name);
                let bytes = vec![0 as u8; length as usize];
                let mut slice = bytes.into_boxed_slice();
                cur.read_exact(&mut slice).unwrap();

                Attribute::Unknown {
                    name_index: name_index,
                    length: length,
                    info: slice,
                }
            }
        }
    }
}
