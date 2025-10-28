
use std::collections::HashMap;

use super::{NativeFn, Value};
use super::instructions::*;

pub struct Runtime {
    program: Program,
    line: usize,
    index: usize,

    natives: HashMap<String, NativeFn>,
    stack: Vec<Value>,
    sp: usize,
}

impl Runtime {
    pub fn new(stack_size: usize) -> Self {
        Self {
            program: Program::new(),
            line: 0,
            index: 0,

            natives: HashMap::new(),
            stack: vec![Value::Nil; stack_size],
            sp: 0,
        }
    }

    pub fn register_function(&mut self, name: String, func: NativeFn) {
        self.natives.insert(name, func);
    }

    pub fn pop_stack(&mut self) -> Value {
        self.stack.pop().unwrap()
    }

    pub fn run(&mut self, program: Program) -> Value {
        self.program = program;
        self.line = 0;
        self.index = 0;

        while self.line < self.program.lines() {
            let line = self.program.get_line_mut(self.line);

            while self.index < line.len() {
                match &line[self.index] {
                    Instruction::Dumpling(dumpling) => match dumpling {
                        Dumpling::Stringify => {
                            let top = self.stack.pop().unwrap();
                            self.stack.push(Value::String(match top {
                                Value::Nil => "nil".to_string(),
                                Value::Int(val) => val.to_string(),
                                Value::Float(val) => val.to_string(),
                                Value::String(val) => val,
                                Value::RawText(text) => text,
                                _ => todo!(),
                            }))
                        },
                        Dumpling::Text(text) => self.stack.push(Value::RawText(text.clone())),
                        _ => todo!("{:?}", dumpling),
                    },
                    Instruction::Other(operation) => {
                        match operation {
                            Operation::Eat(op) => print!("{}", match op {
                                Eat::StackTop => self.stack.pop().unwrap(),
                                _ => todo!(),
                            }),
                        }
                    }
                    _ => panic!("HOW?"),
                }
                self.index += 1;
            }

            self.line += 1;
        }

        Value::Nil
    }
}
