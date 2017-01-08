use super::class::Class;
use super::constant_pool::ConstantPoolEntry;
use super::opcode;
use super::Value;

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

        loop {
            match self.read_u8() {
                opcode::NOP => (),
                // opcode::ACONST_NULL => push!(Value::NullReference),
                opcode::ICONST_M1 => push!(Value::Int(-1)),
                opcode::ICONST_0 => push!(Value::Int(0)),
                opcode::ICONST_1 => push!(Value::Int(1)),
                opcode::ICONST_2 => push!(Value::Int(2)),
                opcode::ICONST_3 => push!(Value::Int(3)),
                opcode::ICONST_4 => push!(Value::Int(4)),
                opcode::ICONST_5 => push!(Value::Int(5)),
                opcode::LCONST_0 => push!(Value::Long(0)),
                opcode::LCONST_1 => push!(Value::Long(1)),
                opcode::FCONST_0 => push!(Value::Float(0.0)),
                opcode::FCONST_1 => push!(Value::Float(1.0)),
                opcode::FCONST_2 => push!(Value::Float(2.0)),
                opcode::DCONST_0 => push!(Value::Double(0.0)),
                opcode::DCONST_1 => push!(Value::Double(1.0)),
                opcode::BIPUSH => {
                    let byte = self.read_u8();
                    push!(Value::Int(byte as i32));
                }
                opcode::SIPUSH => {
                    let short = self.read_u16();
                    push!(Value::Int(short as i32));
                }
                opcode::LDC => {
                    let index = self.read_u8();
                    push!(self.class.get_constant_pool().resolve_literal(index as u16).clone());
                }
                opcode::ISTORE_0 => store!(0),
                opcode::ISTORE_1 => store!(1),
                opcode::ISTORE_2 => store!(2),
                opcode::ISTORE_3 => store!(3),
                opcode::ILOAD_1 => {
                    let local = self.local_variables[1].clone().unwrap();
                    push!(local);
                }
                opcode::IADD => {
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    self.operand_stack.push(Value::Int(val1 + val2));
                }
                opcode::ISUB => {
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    self.operand_stack.push(Value::Int(val1 - val2));
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
