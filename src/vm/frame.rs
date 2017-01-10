use super::class::Class;
use super::constant_pool::ConstantPoolEntry;
use super::opcode;
use super::Value;

use std::f32;
use std::f64;
use std::fmt;
use std::num::Wrapping;

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

    pub fn read_u8(&mut self) -> u8 {
        let result = self.code[self.pc as usize];
        self.pc += 1;
        result
    }

    pub fn read_u16(&mut self) -> u16 {
        ((self.read_u8() as u16) << 8) | (self.read_u8() as u16)
    }

    pub fn run(mut self) -> Option<Value> {
        macro_rules! push {
            ($v: expr) => ({
                self.operand_stack.push($v);
            });
        }

        macro_rules! pop {
            () => (self.operand_stack.pop().unwrap());
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
                push!(local);
            });
        }

        macro_rules! branch {
            ($pc: expr, $offset: expr) => ({
                self.pc = $pc + $offset;
            });
        }

        loop {
            match self.read_u8() {
                opcode::NOP => (),
                // opcode::ACONST_NULL => push!(Value::NullReference),
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
                    push!(self.class.get_constant_pool().resolve_literal(index as u16).clone());
                }
                opcode::LDC_W | opcode::LDC2_W => {
                    let index = self.read_u16();
                    push!(self.class.get_constant_pool().resolve_literal(index).clone());
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
                // TODO: Array stuff
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
                // TODO: Array stuff
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
                            push!(value2);
                            push!(value3);
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
                            push!(value1.clone());
                            push!(value2.clone());
                            push!(value3);
                            push!(value1);
                            push!(value2);
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
                                    push!(value2);
                                    push!(value3);
                                    push!(value1);
                                }
                            }
                        }
                        _ => {
                            let value3 = pop!();
                            match value3 {
                                Value::Long(_) | Value::Double(_) => {
                                    push!(value1.clone());
                                    push!(value2.clone());
                                    push!(value3);
                                    push!(value1);
                                    push!(value2);
                                }
                                _ => {
                                    let value4 = pop!();
                                    push!(value1.clone());
                                    push!(value2.clone());
                                    push!(value3);
                                    push!(value4);
                                    push!(value1);
                                    push!(value2);
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
                    push!(Value::Double(val1 % val2));
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
                    if val1 == f32::NAN || val2 == f32::NAN || val1 < val2 {
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
                    if val1 == f32::NAN || val2 == f32::NAN || val1 > val2 {
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
                    if val1 == f64::NAN || val2 == f64::NAN || val1 < val2 {
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
                    if val1 == f64::NAN || val2 == f64::NAN || val1 > val2 {
                        push!(Value::Double(1.0));
                    } else if val1 < val2 {
                        push!(Value::Double(-1.0));
                    } else {
                        push!(Value::Double(0.0));
                    }
                }
                opcode::IFEQ => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16();
                    if pop!(Value::Int) == Wrapping(0) {
                        branch!(pc, offset);
                    }
                }
                opcode::IFNE => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16();
                    if pop!(Value::Int) != Wrapping(0) {
                        branch!(pc, offset);
                    }
                }
                opcode::IFLT => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16();
                    if pop!(Value::Int) < Wrapping(0) {
                        branch!(pc, offset);
                    }
                }
                opcode::IFGE => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16();
                    if pop!(Value::Int) >= Wrapping(0) {
                        branch!(pc, offset);
                    }
                }
                opcode::IFGT => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16();
                    if pop!(Value::Int) > Wrapping(0) {
                        branch!(pc, offset);
                    }
                }
                opcode::IFLE => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16();
                    if pop!(Value::Int) <= Wrapping(0) {
                        branch!(pc, offset);
                    }
                }
                opcode::IF_ICMPEQ => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16();
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    if val1 == val2 {
                        branch!(pc, offset);
                    }
                }
                opcode::IF_ICMPNE => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16();
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    if val1 != val2 {
                        branch!(pc, offset);
                    }
                }
                opcode::IF_ICMPLT => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16();
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    if val1 < val2 {
                        branch!(pc, offset);
                    }
                }
                opcode::IF_ICMPGE => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16();
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    if val1 >= val2 {
                        branch!(pc, offset);
                    }
                }
                opcode::IF_ICMPGT => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16();
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    if val1 > val2 {
                        branch!(pc, offset);
                    }
                }
                opcode::IF_ICMPLE => {
                    let pc = self.pc - 1; // pc is incremented for each byte read
                    let offset = self.read_u16();
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    if val1 <= val2 {
                        branch!(pc, offset);
                    }
                }
                // TODO: IF_ACMPEQ, IF_ACMPNE
                opcode::INVOKESTATIC => {
                    let index = self.read_u16();
                    if let Some(ConstantPoolEntry::MethodRef(ref symref)) =
                        self.class.get_constant_pool()[index] {
                        // TODO: Actually implement this
                        if symref.sig.name == "println" {
                            println!("{:?}", self.operand_stack.pop().unwrap());
                        } else {
                            panic!("unimplemented");
                        }
                    } else {
                        panic!("invokestatic must refer to a MethodRef");
                    }
                }
                opcode::RETURN => {
                    return None;
                }
                _ => {
                    println!("{:#?}", self);
                    panic!("Unknown instruction at pc {}", self.pc);
                }
            }
        }
    }
}
