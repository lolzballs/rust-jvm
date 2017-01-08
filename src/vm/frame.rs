use super::class::Class;
use super::constant_pool::ConstantPoolEntry;
use super::opcode;
use super::Value;

use std::num::Wrapping;

#[derive(Debug)]
pub struct Frame<'a> {
    class: &'a Class,
    code: &'a [u8],
    pc: u16,
    local_variables: Vec<Option<Value>>,
    operand_stack: Vec<Value>,
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
                self.local_variables[$index as usize] = Some(value);
            });
        }

        macro_rules! load {
            ($index: expr) => ({
                let local = self.local_variables[$index as usize].clone().unwrap();
                push!(local);
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
                // TODO: LDC_W and LDC2_W
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
                // TODO: Array stuff
                opcode::POP => {
                    pop!();
                }
                // TODO: POP2
                opcode::DUP => {
                    let operand = self.operand_stack.last().unwrap().clone();
                    push!(operand);
                }
                // TODO: DUP_X1 and DUP_X2
                // TODO: DUP2, DUP2_X1, and DUP2_X2
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
