

pub mod instructions;
pub mod runtime;
pub mod stdlib;

use std::collections::HashMap;

use instructions::{Dumpling, Eat, Instruction, Operation, Program};
use runtime::Runtime;

type NativeFn = fn(&mut Vec<Value>) -> Value;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,
    Int(i64),
    Float(f64),
    String(String),
    NativeFn {
        name: String,
        func: NativeFn,
    },
    RawText(String),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "()"),
            Self::Int(val) => write!(f, "{}", val),
            Self::Float(val) => write!(f, "{}", val),
            Self::String(val) => write!(f, "{}", val),
            Self::RawText(val) => write!(f, "({})", val),
            _ => todo!(),
        }
    }
}

impl Runtime {
    pub fn run(&mut self, program: Program) -> Value {
        self.program = program;
        self.line = 0;
        self.index = 0;

        while self.line < self.program.lines() {
            let line = self.program.get_line_mut(self.line);

            while self.index < line.len() {
                match &line[self.index] {
                    Instruction::Dumpling(dumpling) => runtime_interpret_dumpling(dumpling, &mut self.line, &mut self.index, &mut self.stack, &mut self.natives),
                    Instruction::Other(operation) => runtime_interpret_other(operation, &mut self.line, &mut self.index, &mut self.stack, &mut self.natives),
                    _ => panic!("HOW?"),
                }
            }

            self.line += 1;
        }

        Value::Nil
    }
}

// Since you won't let me nicely cut it up into methods and I saw someone using
// an "associated function", I'll just do this instead.
pub(crate) fn runtime_interpret_dumpling(dumpling: &Dumpling, line: &mut usize, index: &mut usize, stack: &mut Vec<Value>, natives: &mut HashMap<String, NativeFn>) {
    match dumpling {
        Dumpling::Float(val) => stack.push(Value::Float(*val)),
        Dumpling::FnCall(name) => {
            let Some(function) = natives.get(name) else {
                panic!("Function ':{name}' does not exist");
            };

            let val = function(stack);
            stack.push(val);
        }
        Dumpling::Int(val) => stack.push(Value::Int(*val)),
        Dumpling::Jump => {
            let amount = stack.pop().unwrap();

            match amount {
                Value::Int(offset) => {
                    *line = offset as usize - 1;
                    *index = 0;
                    return;
                },
                _ => panic!("Cannot jump using a non-integer amount"),
            }
        },
        Dumpling::Stringify => {
            let top = stack.pop().unwrap();
            stack.push(Value::String(match top {
                Value::Nil => "nil".to_string(),
                Value::Int(val) => val.to_string(),
                Value::Float(val) => val.to_string(),
                Value::String(val) => val,
                Value::NativeFn {
                    name,
                    func: _
                } => format!(":{}", name),
                Value::RawText(text) => text,
                _ => todo!(),
            }))
        },
        Dumpling::Text(text) => stack.push(Value::RawText(text.clone())),
        _ => todo!("{:?}", dumpling),
    }
    *index += 1;
}

pub(crate) fn runtime_interpret_other(operation: &Operation, line: &mut usize, index: &mut usize, stack: &mut Vec<Value>, natives: &mut HashMap<String, NativeFn>) {
    match operation {
        Operation::Eat(op) => print!("{}", match op {
            Eat::StackTop => stack.pop().unwrap(),
            _ => todo!(),
        }),
    }
    *index += 1;
}
