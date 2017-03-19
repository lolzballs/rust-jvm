use super::class::Class;
use super::class_loader::ClassLoader;
use super::constant_pool::ConstantPoolEntry;
use super::opcode;
use super::sig;
use super::value;
use super::value::Value;

use std::cell::RefCell;
use std::f32;
use std::f64;
use std::fmt;
use std::num::Wrapping;
use std::rc::Rc;

pub struct Frame<'a> {
    class: &'a Class,
    code: &'a [u8],
    pc: u16,
    local_variables: Vec<Option<Value>>,
    operand_stack: Vec<Value>,
}

impl<'a> fmt::Debug for Frame<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Frame")
            .field("code", &self.code)
            .field("pc", &self.pc)
            .field("local_variables", &self.local_variables)
            .field("operand_stack", &self.operand_stack)
            .finish()
    }
}

impl<'a> Frame<'a> {
    pub fn new(class: &'a Class, code: &'a [u8], local_variables: Vec<Option<Value>>) -> Self {
        Frame {
            class: class,
            code: code,
            pc: 0,
            local_variables: local_variables,
            operand_stack: vec![],
        }
    }

    fn read_u8(&mut self) -> u8 {
        let result = self.code[self.pc as usize];
        self.pc += 1;
        result
    }

    fn read_u16(&mut self) -> u16 {
        ((self.read_u8() as u16) << 8) | (self.read_u8() as u16)
    }

    fn read_u32(&mut self) -> u32 {
        ((self.read_u8() as u32) << 24) | ((self.read_u8() as u32) << 16) |
        ((self.read_u8() as u32) << 8) | (self.read_u8() as u32)
    }

    fn pop_count(&mut self, count: usize) -> Vec<Value> {
        let start = self.operand_stack.len() - count;
        self.operand_stack.drain(start..).collect()
    }

    pub fn run(mut self, class_loader: &mut ClassLoader) -> Option<Value> {
        macro_rules! push {
            ($v: expr) => ({
                self.operand_stack.push($v);
            });
        }

        macro_rules! pop {
            () => (self.operand_stack.pop().unwrap_or_else(|| panic!("{:?}", self.code)));
            ($value_variant: path) => ({
                match pop!() {
                    $value_variant(v) => v,
                    v => panic!("Expected to pop a value of type {}, but was {:?}",
                                stringify!($value_variant), v),
                }
            });
        }

        macro_rules! store {
            ($index: expr) => ({
                let value = pop!();
                // the local variables at index and index+1 are set to value for long and double
                match value {
                    Value::Long(_) | Value::Double(_) => {
                        self.local_variables[($index + 1) as usize] = None;
                    },
                    _ => ()
                }
                self.local_variables[$index as usize] = Some(value);
            });
        }

        macro_rules! load {
            ($index: expr) => ({
                let local = self.local_variables[$index as usize].clone().unwrap();
                //println!("{:?}", self.operand_stack);
                push!(local);
            });
        }

        macro_rules! branch {
            ($pc: expr, $offset: expr) => ({
                self.pc = ($pc as i16 + $offset as i16) as u16;
            });
        }

        loop {
            match self.read_u8() {
                opcode::NOP => (),
                opcode::ACONST_NULL => push!(Value::NullReference),
                opcode::ICONST_M1 => push!(Value::Int(Wrapping(-1))),
                opcode::ICONST_0 => push!(Value::Int(Wrapping(0))),
                opcode::ICONST_1 => push!(Value::Int(Wrapping(1))),
                opcode::ICONST_2 => push!(Value::Int(Wrapping(2))),
                opcode::ICONST_3 => push!(Value::Int(Wrapping(3))),
                opcode::ICONST_4 => push!(Value::Int(Wrapping(4))),
                opcode::ICONST_5 => push!(Value::Int(Wrapping(5))),
                opcode::LCONST_0 => push!(Value::Long(Wrapping(0))),
                opcode::LCONST_1 => push!(Value::Long(Wrapping(1))),
                opcode::FCONST_0 => push!(Value::Float(0.0)),
                opcode::FCONST_1 => push!(Value::Float(1.0)),
                opcode::FCONST_2 => push!(Value::Float(2.0)),
                opcode::DCONST_0 => push!(Value::Double(0.0)),
                opcode::DCONST_1 => push!(Value::Double(1.0)),
                opcode::BIPUSH => {
                    let byte = self.read_u8();
                    push!(Value::Int(Wrapping((byte as i8) as i32)));
                }
                opcode::SIPUSH => {
                    let short = self.read_u16();
                    push!(Value::Int(Wrapping((short as i16) as i32)));
                }
                opcode::LDC => {
                    let index = self.read_u8();
                    push!(self.class
                        .get_constant_pool()
                        .resolve_literal(index as u16, class_loader)
                        .clone());
                }
                opcode::LDC_W | opcode::LDC2_W => {
                    let index = self.read_u16();
                    push!(self.class
                        .get_constant_pool()
                        .resolve_literal(index, class_loader)
                        .clone());
                }
                opcode::ILOAD | opcode::LLOAD | opcode::FLOAD | opcode::DLOAD | opcode::ALOAD => {
                    let index = self.read_u8();
                    load!(index);
                }
                opcode::ILOAD_0 | opcode::LLOAD_0 | opcode::FLOAD_0 | opcode::DLOAD_0 |
                opcode::ALOAD_0 => load!(0),
                opcode::ILOAD_1 | opcode::LLOAD_1 | opcode::FLOAD_1 | opcode::DLOAD_1 |
                opcode::ALOAD_1 => load!(1),
                opcode::ILOAD_2 | opcode::LLOAD_2 | opcode::FLOAD_2 | opcode::DLOAD_2 |
                opcode::ALOAD_2 => load!(2),
                opcode::ILOAD_3 | opcode::LLOAD_3 | opcode::FLOAD_3 | opcode::DLOAD_3 |
                opcode::ALOAD_3 => load!(3),
                opcode::IALOAD | opcode::LALOAD | opcode::FALOAD | opcode::DALOAD |
                opcode::AALOAD | opcode::BALOAD | opcode::CALOAD | opcode::SALOAD => {
                    let index = pop!(Value::Int).0 as usize;
                    let arrayref = pop!(Value::ArrayReference);
                    push!(arrayref.borrow().get(index));
                }
                opcode::ISTORE | opcode::LSTORE | opcode::FSTORE | opcode::DSTORE |
                opcode::ASTORE => {
                    let index = self.read_u8();
                    store!(index);
                }
                opcode::ISTORE_0 | opcode::LSTORE_0 | opcode::FSTORE_0 | opcode::DSTORE_0 |
                opcode::ASTORE_0 => store!(0),
                opcode::ISTORE_1 | opcode::LSTORE_1 | opcode::FSTORE_1 | opcode::DSTORE_1 |
                opcode::ASTORE_1 => store!(1),
                opcode::ISTORE_2 | opcode::LSTORE_2 | opcode::FSTORE_2 | opcode::DSTORE_2 |
                opcode::ASTORE_2 => store!(2),
                opcode::ISTORE_3 | opcode::LSTORE_3 | opcode::FSTORE_3 | opcode::DSTORE_3 |
                opcode::ASTORE_3 => store!(3),
                opcode::IASTORE | opcode::LASTORE | opcode::FASTORE | opcode::DASTORE |
                opcode::AASTORE | opcode::BASTORE | opcode::CASTORE | opcode::SASTORE => {
                    let value = pop!();
                    let index = pop!(Value::Int).0 as usize;
                    let arrayref = pop!(Value::ArrayReference);
                    arrayref.borrow_mut().insert(index, value);
                }
                opcode::POP => {
                    pop!();
                }
                opcode::POP2 => {
                    match pop!() {
                        Value::Long(_) | Value::Double(_) => (),
                        _ => {
                            pop!();
                        }
                    };
                }
                opcode::DUP => {
                    let operand = self.operand_stack.last().unwrap().clone();
                    push!(operand);
                }
                opcode::DUP_X1 => {
                    let value1 = pop!();
                    let value2 = pop!();
                    push!(value1.clone());
                    push!(value2);
                    push!(value1);
                }
                opcode::DUP_X2 => {
                    let value1 = pop!();
                    let value2 = pop!();
                    match value2 {
                        Value::Long(_) | Value::Double(_) => {
                            push!(value1.clone());
                            push!(value2);
                            push!(value1);
                        }
                        _ => {
                            let value3 = pop!();
                            push!(value1.clone());
                            push!(value3);
                            push!(value2);
                            push!(value1);
                        }
                    }
                }
                opcode::DUP2 => {
                    let value1 = pop!();
                    match value1 {
                        Value::Long(_) | Value::Double(_) => {
                            push!(value1.clone());
                            push!(value1);
                        }
                        _ => {
                            let value2 = pop!();
                            push!(value1.clone());
                            push!(value2.clone());
                            push!(value1);
                            push!(value2);
                        }
                    }
                }
                opcode::DUP2_X1 => {
                    let value1 = pop!();
                    let value2 = pop!();
                    match value1 {
                        Value::Long(_) | Value::Double(_) => {
                            push!(value1.clone());
                            push!(value2);
                            push!(value1);
                        }
                        _ => {
                            let value3 = pop!();
                            push!(value2.clone());
                            push!(value1.clone());
                            push!(value3);
                            push!(value2);
                            push!(value1);
                        }
                    }
                }
                opcode::DUP2_X2 => {
                    let value1 = pop!();
                    let value2 = pop!();
                    match value1 {
                        Value::Long(_) | Value::Double(_) => {
                            match value2 {
                                Value::Long(_) | Value::Double(_) => {
                                    push!(value1.clone());
                                    push!(value2);
                                    push!(value1);
                                }
                                _ => {
                                    let value3 = pop!();
                                    push!(value1.clone());
                                    push!(value3);
                                    push!(value2);
                                    push!(value1);
                                }
                            }
                        }
                        _ => {
                            let value3 = pop!();
                            match value3 {
                                Value::Long(_) | Value::Double(_) => {
                                    push!(value2.clone());
                                    push!(value1.clone());
                                    push!(value3);
                                    push!(value2);
                                    push!(value1);
                                }
                                _ => {
                                    let value4 = pop!();
                                    push!(value2.clone());
                                    push!(value1.clone());
                                    push!(value4);
                                    push!(value3);
                                    push!(value2);
                                    push!(value1);
                                }
                            }
                        }
                    }
                }
                opcode::SWAP => {
                    let val2 = pop!();
                    let val1 = pop!();
                    push!(val2);
                    push!(val1);
                }
                opcode::IADD => {
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    push!(Value::Int(val1 + val2));
                }
                opcode::LADD => {
                    let val2 = pop!(Value::Long);
                    let val1 = pop!(Value::Long);
                    push!(Value::Long(val1 + val2));
                }
                opcode::FADD => {
                    let val2 = pop!(Value::Float);
                    let val1 = pop!(Value::Float);
                    push!(Value::Float(val1 + val2));
                }
                opcode::DADD => {
                    let val2 = pop!(Value::Double);
                    let val1 = pop!(Value::Double);
                    push!(Value::Double(val1 + val2));
                }
                opcode::ISUB => {
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    push!(Value::Int(val1 - val2));
                }
                opcode::LSUB => {
                    let val2 = pop!(Value::Long);
                    let val1 = pop!(Value::Long);
                    push!(Value::Long(val1 - val2));
                }
                opcode::FSUB => {
                    let val2 = pop!(Value::Float);
                    let val1 = pop!(Value::Float);
                    push!(Value::Float(val1 - val2));
                }
                opcode::DSUB => {
                    let val2 = pop!(Value::Double);
                    let val1 = pop!(Value::Double);
                    push!(Value::Double(val1 - val2));
                }
                opcode::IMUL => {
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    push!(Value::Int(val1 * val2));
                }
                opcode::LMUL => {
                    let val2 = pop!(Value::Long);
                    let val1 = pop!(Value::Long);
                    push!(Value::Long(val1 * val2));
                }
                opcode::FMUL => {
                    let val2 = pop!(Value::Float);
                    let val1 = pop!(Value::Float);
                    push!(Value::Float(val1 * val2));
                }
                opcode::DMUL => {
                    let val2 = pop!(Value::Double);
                    let val1 = pop!(Value::Double);
                    push!(Value::Double(val1 * val2));
                }
                opcode::IDIV => {
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    push!(Value::Int(val1 / val2));
                }
                opcode::LDIV => {
                    let val2 = pop!(Value::Long);
                    let val1 = pop!(Value::Long);
                    push!(Value::Long(val1 / val2));
                }
                opcode::FDIV => {
                    let val2 = pop!(Value::Float);
                    let val1 = pop!(Value::Float);
                    push!(Value::Float(val1 / val2));
                }
                opcode::DDIV => {
                    let val2 = pop!(Value::Double);
                    let val1 = pop!(Value::Double);
                    push!(Value::Double(val1 / val2));
                }
                opcode::IREM => {
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    push!(Value::Int(val1 % val2));
                }
                opcode::LREM => {
                    let val2 = pop!(Value::Long);
                    let val1 = pop!(Value::Long);
                    push!(Value::Long(val1 % val2));
                }
                opcode::FREM => {
                    let val2 = pop!(Value::Float);
                    let val1 = pop!(Value::Float);
                    push!(Value::Float(val1 % val2));
                }
                opcode::DREM => {
                    let val2 = pop!(Value::Double);
                    let val1 = pop!(Value::Double);
                    push!(Value::Double(val1 % val2));
                }
                opcode::INEG => {
                    let val1 = pop!(Value::Int);
                    push!(Value::Int(-val1));
                }
                opcode::LNEG => {
                    let val1 = pop!(Value::Long);
                    push!(Value::Long(-val1));
                }
                opcode::FNEG => {
                    let val1 = pop!(Value::Float);
                    push!(Value::Float(-val1));
                }
                opcode::DNEG => {
                    let val1 = pop!(Value::Double);
                    push!(Value::Double(-val1));
                }
                opcode::ISHL => {
                    let Wrapping(shift) = pop!(Value::Int);
                    let value = pop!(Value::Int);
                    push!(Value::Int(value << (shift & 0x1F) as usize));
                }
                opcode::LSHL => {
                    let Wrapping(shift) = pop!(Value::Int);
                    let value = pop!(Value::Long);
                    push!(Value::Long(value << (shift & 0x3F) as usize));
                }
                opcode::ISHR => {
                    let Wrapping(shift) = pop!(Value::Int);
                    let value = pop!(Value::Int);
                    push!(Value::Int(value >> (shift & 0x1F) as usize));
                }
                opcode::LSHR => {
                    let Wrapping(shift) = pop!(Value::Int);
                    let value = pop!(Value::Long);
                    push!(Value::Long(value >> (shift & 0x3F) as usize));
                }
                // TODO: Check correctness for logical shifts
                opcode::IUSHR => {
                    let Wrapping(shift) = pop!(Value::Int);
                    let value = pop!(Value::Int).0 as u32;
                    push!(Value::Int(Wrapping((value << (shift & 0x1F) as usize) as i32)));
                }
                opcode::LUSHR => {
                    let Wrapping(shift) = pop!(Value::Int);
                    let value = pop!(Value::Long).0 as u64;
                    push!(Value::Long(Wrapping((value << (shift & 0x3F) as usize) as i64)));
                }
                opcode::IAND => {
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    push!(Value::Int(val1 & val2));
                }
                opcode::LAND => {
                    let val2 = pop!(Value::Long);
                    let val1 = pop!(Value::Long);
                    push!(Value::Long(val1 & val2));
                }
                opcode::IOR => {
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    push!(Value::Int(val1 | val2));
                }
                opcode::LOR => {
                    let val2 = pop!(Value::Long);
                    let val1 = pop!(Value::Long);
                    push!(Value::Long(val1 | val2));
                }
                opcode::IXOR => {
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    push!(Value::Int(val1 ^ val2));
                }
                opcode::LXOR => {
                    let val2 = pop!(Value::Long);
                    let val1 = pop!(Value::Long);
                    push!(Value::Long(val1 ^ val2));
                }
                opcode::IINC => {
                    let index = self.read_u8();
                    let const_incr = (self.read_u8() as i8) as i32;
                    match self.local_variables[index as usize] {
                        Some(Value::Int(ref mut value)) => {
                            *value += Wrapping(const_incr);
                        }
                        _ => panic!("Cannot IINC on non-integer at index: {}", index),
                    };
                }
                opcode::I2L => {
                    match pop!() {
                        Value::Int(v) => push!(Value::Long(Wrapping(v.0 as i64))),
                        v => panic!("Expected Int, got {:?}", v), 
                    };
                }
                opcode::I2F => {
                    match pop!() {
                        Value::Int(v) => push!(Value::Float(v.0 as f32)),
                        v => panic!("Expected Int, got {:?}", v), 
                    };
                }
                opcode::I2D => {
                    match pop!() {
                        Value::Int(v) => push!(Value::Double(v.0 as f64)),
                        v => panic!("Expected Int, got {:?}", v), 
                    };
                }
                opcode::L2I => {
                    match pop!() {
                        Value::Long(v) => push!(Value::Int(Wrapping(v.0 as i32))),
                        v => panic!("Expected Long, got {:?}", v), 
                    };
                }
                opcode::L2F => {
                    match pop!() {
                        Value::Long(v) => push!(Value::Float(v.0 as f32)),
                        v => panic!("Expected Long, got {:?}", v), 
                    };
                }
                opcode::L2D => {
                    match pop!() {
                        Value::Long(v) => push!(Value::Double(v.0 as f64)),
                        v => panic!("Expected Long, got {:?}", v), 
                    };
                }
                opcode::F2I => {
                    match pop!() {
                        Value::Float(v) => push!(Value::Int(Wrapping(v as i32))),
                        v => panic!("Expected Float, got {:?}", v), 
                    };
                }
                opcode::F2L => {
                    match pop!() {
                        Value::Float(v) => push!(Value::Long(Wrapping(v as i64))),
                        v => panic!("Expected Float, got {:?}", v), 
                    };
                }
                opcode::F2D => {
                    match pop!() {
                        Value::Float(v) => push!(Value::Double(v as f64)),
                        v => panic!("Expected Float, got {:?}", v), 
                    };
                }
                opcode::D2I => {
                    match pop!() {
                        Value::Double(v) => push!(Value::Int(Wrapping(v as i32))),
                        v => panic!("Expected Double, got {:?}", v), 
                    };
                }
                opcode::D2L => {
                    match pop!() {
                        Value::Double(v) => push!(Value::Long(Wrapping(v as i64))),
                        v => panic!("Expected Double, got {:?}", v), 
                    };
                }
                opcode::D2F => {
                    match pop!() {
                        Value::Double(v) => push!(Value::Double(v as f64)),
                        v => panic!("Expected Double, got {:?}", v), 
                    };
                }
                // TODO: Check if these narrowing conversions are valid
                opcode::I2B => {
                    match pop!() {
                        Value::Int(v) => push!(Value::Int(Wrapping(v.0 as i8 as i32))),
                        v => panic!("Expected Int, got {:?}", v), 
                    };
                }
                opcode::I2C => {
                    match pop!() {
                        Value::Int(v) => push!(Value::Int(Wrapping(v.0 as u16 as i32))),
                        v => panic!("Expected Int, got {:?}", v), 
                    };
                }
                opcode::I2S => {
                    match pop!() {
                        Value::Int(v) => push!(Value::Int(Wrapping(v.0 as i16 as i32))),
                        v => panic!("Expected Int, got {:?}", v), 
                    };
                }
                opcode::LCMP => {
                    let val2 = pop!(Value::Long);
                    let val1 = pop!(Value::Long);
                    if val1 > val2 {
                        push!(Value::Int(Wrapping(1)));
                    } else if val1 < val2 {
                        push!(Value::Int(Wrapping(-1)));
                    } else {
                        push!(Value::Int(Wrapping(0)));
                    }
                }
                opcode::FCMPL => {
                    let val2 = pop!(Value::Float);
                    let val1 = pop!(Value::Float);
                    if val1.is_nan() || val2.is_nan() || val1 < val2 {
                        push!(Value::Float(-1.0));
                    } else if val1 > val2 {
                        push!(Value::Float(1.0));
                    } else {
                        push!(Value::Float(0.0));
                    }
                }
                opcode::FCMPG => {
                    let val2 = pop!(Value::Float);
                    let val1 = pop!(Value::Float);
                    if val1.is_nan() || val2.is_nan() || val1 > val2 {
                        push!(Value::Float(1.0));
                    } else if val1 < val2 {
                        push!(Value::Float(-1.0));
                    } else {
                        push!(Value::Float(0.0));
                    }
                }
                opcode::DCMPL => {
                    let val2 = pop!(Value::Double);
                    let val1 = pop!(Value::Double);
                    if val1.is_nan() || val2.is_nan() || val1 < val2 {
                        push!(Value::Double(-1.0));
                    } else if val1 > val2 {
                        push!(Value::Double(1.0));
                    } else {
                        push!(Value::Double(0.0));
                    }
                }
                opcode::DCMPG => {
                    let val2 = pop!(Value::Double);
                    let val1 = pop!(Value::Double);
                    if val1.is_nan() || val2.is_nan() || val1 > val2 {
                        push!(Value::Double(1.0));
                    } else if val1 < val2 {
                        push!(Value::Double(-1.0));
                    } else {
                        push!(Value::Double(0.0));
                    }
                }
                opcode::IFEQ => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16() as i16;
                    let pop = pop!(Value::Int);
                    if pop == Wrapping(0) {
                        branch!(pc, offset);
                    }
                }
                opcode::IFNE => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16() as i16;
                    let pop = pop!(Value::Int);
                    if pop != Wrapping(0) {
                        branch!(pc, offset);
                    }
                }
                opcode::IFLT => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16() as i16;
                    let pop = pop!(Value::Int);
                    if pop < Wrapping(0) {
                        branch!(pc, offset);
                    }
                }
                opcode::IFGE => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16() as i16;
                    let pop = pop!(Value::Int);
                    if pop >= Wrapping(0) {
                        branch!(pc, offset);
                    }
                }
                opcode::IFGT => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16() as i16;
                    let pop = pop!(Value::Int);
                    if pop > Wrapping(0) {
                        branch!(pc, offset);
                    }
                }
                opcode::IFLE => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16() as i16;
                    let pop = pop!(Value::Int);
                    if pop <= Wrapping(0) {
                        branch!(pc, offset);
                    }
                }
                opcode::IF_ICMPEQ => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16() as i16;
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    if val1 == val2 {
                        branch!(pc, offset);
                    }
                }
                opcode::IF_ICMPNE => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16() as i16;
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    if val1 != val2 {
                        branch!(pc, offset);
                    }
                }
                opcode::IF_ICMPLT => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16() as i16;
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    if val1 < val2 {
                        branch!(pc, offset);
                    }
                }
                opcode::IF_ICMPGE => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16() as i16;
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    if val1 >= val2 {
                        branch!(pc, offset);
                    }
                }
                opcode::IF_ICMPGT => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16() as i16;
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    if val1 > val2 {
                        branch!(pc, offset);
                    }
                }
                opcode::IF_ICMPLE => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16() as i16;
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    if val1 <= val2 {
                        branch!(pc, offset);
                    }
                }
                // TODO: IF_ACMPEQ, IF_ACMPNE
                opcode::GOTO => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16() as i16;
                    branch!(pc, offset);
                }
                opcode::JSR => {
                    panic!("JSR is not supported in this jvm");
                }
                opcode::RET => {
                    panic!("RET is not supported in this jvm");
                }
                opcode::TABLESWITCH => {
                    let pc = self.pc - 1;
                    // Align
                    while self.pc % 4 != 0 {
                        self.pc += 1;
                    }
                    let default = self.read_u32() as i32;
                    let low = self.read_u32() as i32;
                    let high = self.read_u32() as i32;

                    let size = (high - low + 1) as usize;
                    let mut offsets = vec![0 as u32; size];
                    for _ in 0..size {
                        offsets.push(self.read_u32());
                    }

                    let index = pop!(Value::Int).0;
                    if index < low || index > high {
                        branch!(pc, default);
                    } else {
                        branch!(pc, offsets[(index - low) as usize]);
                    }
                }
                // TODO: OPTIMIZE by transmuting bytes into i32s, then binary search
                opcode::LOOKUPSWITCH => {
                    let pc = self.pc - 1;
                    // Align
                    while self.pc % 4 != 0 {
                        self.pc += 1;
                    }
                    let default = self.read_u32() as i32;
                    let npairs = self.read_u32() as i32 as usize;

                    let key = pop!(Value::Int).0;
                    let mut found = false;

                    for _ in 0..npairs {
                        let case = self.read_u32() as i32;
                        let offset = self.read_u32() as i32;
                        if key == case {
                            branch!(pc, offset);
                            found = true;
                            break;
                        }
                    }

                    if !found {
                        branch!(pc, default);
                    }
                }
                opcode::IRETURN | opcode::LRETURN | opcode::FRETURN | opcode::DRETURN |
                opcode::ARETURN => {
                    return Some(pop!());
                }
                opcode::RETURN => {
                    return None;
                }
                opcode::GETSTATIC => {
                    let index = self.read_u16();
                    if let Some(ConstantPoolEntry::FieldRef(ref symref)) =
                        self.class.get_constant_pool()[index] {
                        let owning_class = class_loader.resolve_class(&symref.class.sig);
                        let value = owning_class.get_field(class_loader, symref);
                        push!(value);
                    } else {
                        panic!("GETSTATIC {} must point to a FieldRef", index);
                    }
                }
                opcode::PUTSTATIC => {
                    let index = self.read_u16();
                    if let Some(ConstantPoolEntry::FieldRef(ref symref)) =
                        self.class.get_constant_pool()[index] {
                        let value = pop!();
                        let owning_class = class_loader.resolve_class(&symref.class.sig);
                        owning_class.put_field(class_loader, symref, value);
                    } else {
                        panic!("PUTSTATIC {} must point to a FieldRef", index);
                    }
                }
                opcode::GETFIELD => {
                    let index = self.read_u16();
                    if let Some(ConstantPoolEntry::FieldRef(ref symref)) =
                        self.class.get_constant_pool()[index] {
                        match pop!() {
                            Value::Reference(object) => {
                                let value = object.borrow().get_field(&symref.sig);
                                push!(value);
                            }
                            v => panic!("TODO: Some kind of implementation for this: {:?}", v),
                        }
                    } else {
                        panic!("GETFIELD {} must point to a FieldRef", index);
                    }
                }
                opcode::PUTFIELD => {
                    let index = self.read_u16();
                    if let Some(ConstantPoolEntry::FieldRef(ref symref)) =
                        self.class.get_constant_pool()[index] {
                        let value = pop!();
                        match pop!() {
                            Value::Reference(object) => {
                                object.borrow_mut().put_field(symref.sig.clone(), value);
                            }
                            v => panic!("TODO: Some kind of implementation for this: {:?}", v),
                        }
                    } else {
                        panic!("PUTFIELD {} must point to a FieldRef", index);
                    }
                }
                opcode::INVOKEVIRTUAL => {
                    // TODO: Polymorphic invokevirtual
                    let index = self.read_u16();
                    if let Some(ConstantPoolEntry::MethodRef(ref symref)) =
                        self.class.get_constant_pool()[index] {
                        let num_args = symref.sig.params.len();
                        let args = self.pop_count(num_args + 1); // include objectref

                        let owning_class = class_loader.resolve_class(&symref.class.sig);
                        let method = owning_class.find_method(class_loader, symref);

                        let result = method.borrow()
                            .invoke(owning_class.as_ref(), class_loader, Some(args));
                        match result {
                            None => (),
                            Some(value) => push!(value),
                        }
                    } else {
                        panic!("invokevirtual must refer to a MethodRef");
                    }
                }
                opcode::INVOKESPECIAL => {
                    let index = self.read_u16();
                    if let Some(ConstantPoolEntry::MethodRef(ref symref)) =
                        self.class.get_constant_pool()[index] {
                        let num_args = symref.sig.params.len();
                        let args = self.pop_count(num_args + 1); // include objectref

                        let owning_class = class_loader.resolve_class(&symref.class.sig);
                        let method = owning_class.find_method(class_loader, symref);

                        let result = method.borrow()
                            .invoke(owning_class.as_ref(), class_loader, Some(args));
                        match result {
                            None => (),
                            Some(value) => push!(value),
                        }
                    } else {
                        panic!("invokespecial must refer to a MethodRef");
                    }
                }
                opcode::INVOKESTATIC => {
                    let index = self.read_u16();
                    if let Some(ConstantPoolEntry::MethodRef(ref symref)) =
                        self.class.get_constant_pool()[index] {
                        let owning_class = class_loader.resolve_class(&symref.class.sig);
                        let method = owning_class.find_method(class_loader, symref);
                        let num_args = symref.sig.params.len();
                        let args = self.pop_count(num_args);

                        let result = method.borrow()
                            .invoke(owning_class.as_ref(), class_loader, Some(args));
                        match result {
                            None => (),
                            Some(value) => push!(value),
                        }
                    } else {
                        panic!("invokestatic must refer to a MethodRef");
                    }
                }
                // TODO: A bunch of stuff
                opcode::NEW => {
                    let index = self.read_u16();
                    if let Some(ConstantPoolEntry::ClassRef(ref symref)) =
                        self.class.get_constant_pool()[index] {
                        let class = class_loader.resolve_class(&symref.sig);
                        push!(Value::Reference(Rc::new(RefCell::new(value::Scalar::new(class)))));
                    } else {
                        panic!("new must refer to a ClassRef");
                    }
                }
                opcode::NEWARRAY => {
                    let atype = match self.read_u8() {
                        4 => sig::Type::Boolean,
                        5 => sig::Type::Char,
                        6 => sig::Type::Float,
                        7 => sig::Type::Double,
                        8 => sig::Type::Byte,
                        9 => sig::Type::Short,
                        10 => sig::Type::Int,
                        11 => sig::Type::Long,
                        _ => panic!("Unknown array type"),
                    };

                    let count = pop!(Value::Int).0;

                    let class_sig = sig::Class::Array(Box::new(atype));
                    let class = class_loader.resolve_class(&class_sig);
                    let array = value::Array::new(class, count);
                    push!(Value::ArrayReference(Rc::new(RefCell::new(array))));
                }
                opcode::ARRAYLENGTH => {
                    let array_ref = pop!(Value::ArrayReference);
                    push!(Value::Int(Wrapping(array_ref.borrow().len())));
                }
                ins => {
                    println!("{:#?}", self.class);
                    panic!("Unknown instruction at pc {}: {:X}", self.pc, ins);
                }
            }
        }
    }
}
