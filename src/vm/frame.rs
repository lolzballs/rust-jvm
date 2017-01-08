use super::class::Class;
use super::Value;
use super::constant_pool::ConstantPoolEntry;

#[derive(Debug)]
pub struct Frame<'a> {
    class: &'a Class,
    code: &'a [u8],
    pc: u16,
    local_variables: Vec<Option<Value>>,
    operand_stack: Vec<Value>
}

impl<'a> Frame<'a> {
    pub fn new(class: &'a Class,
               code: &'a [u8],
               local_variables: Vec<Option<Value>>) -> Self {
        Frame {
            class: class,
            code: code,
            pc: 0,
            local_variables: local_variables,
            operand_stack: vec![]
        }
    }

    pub fn read_u8(&mut self) -> u8 {
        let result = self.code[self.pc as usize];
        self.pc += 1;
        result
    }

    pub fn read_u16(&mut self) -> u16 {
        ((self.read_u8() as u16) << 8 ) | (self.read_u8() as u16)
    }

    pub fn run(mut self) -> Option<Value> {
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

        loop {
            match self.read_u8() {
                0x05 => { // iconst_2
                    self.operand_stack.push(Value::Int(2));
                },
                0x3C => { // istore_1
                    self.local_variables[1] = self.operand_stack.pop();
                },
                0x10 => { // bipush
                    let byte = self.read_u8();
                    self.operand_stack.push(Value::Int(byte as i32));
                },
                0x1B => { // iload_1
                    let local = self.local_variables[1].clone().unwrap();
                    self.operand_stack.push(local);
                },
                0x60 => { // iadd
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    self.operand_stack.push(Value::Int(val1 + val2));
                },
                0x64 => { // isub
                    let val2 = pop!(Value::Int);
                    let val1 = pop!(Value::Int);
                    self.operand_stack.push(Value::Int(val1 - val2));
                },
                0xB8 => { // invokestatic
                    let index = self.read_u16();
                    if let Some(ConstantPoolEntry::MethodRef(ref symref)) 
                        = self.class.get_constant_pool()[index] {
                        // TODO: Actually implement this
                        if symref.sig.name == "println" {
                            println!("{:?}", self.operand_stack.pop().unwrap());
                        } else {
                            panic!("unimplemented");
                        }
                    } else {
                        panic!("invokestatic must refer to a MethodRef");
                    }
                },
                0xB1 => { // return
                    return None
                },
                _ => {
                    println!("{:#?}", self);
                    panic!("Unknown instruction at pc {}", self.pc); 
                }
            }
        }
    }
}
