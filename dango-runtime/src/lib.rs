

pub mod instructions;
pub mod runtime;
pub mod stdlib;

use std::{collections::HashMap, io::{Write, stdout}};

use instructions::{Dumpling, Instruction, Operation, Program};
use runtime::Runtime;

type NativeFn = fn(&mut Vec<Value>) -> Result<Value, RuntimeError>;

pub enum RuntimeError {
    CustomError(String),
    NonexistentFunction(String),
    StackUnderflow,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CustomError(message) => write!(f, "{}", message),
            Self::NonexistentFunction(name) => write!(f, "function `:{}` does not exist", name),
            Self::StackUnderflow => write!(f, "the stack cannot be popped from when it is already empty"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Nil,                    // nil, but ()
    Int(i64),               // i64
    Float(f64),             // f64
    String(String),         // stringified value
    NativeFn {              // native function handle
        name: String,
        func: NativeFn,
    },
    RawText(String),        // raw text
    Dango(Vec<Value>),      // array-like
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "()"),
            Self::Int(val) => write!(f, "{}", val),
            Self::Float(val) => write!(f, "{}", val),
            Self::String(val) => write!(f, "{}", val),
            Self::RawText(val) => write!(f, "({})", val),
            Self::Dango(dumplings) => {
                for dumpling in dumplings {
                    write!(f, "({})", dumpling)?;
                }
                write!(f, "----")
            },
            _ => todo!(),
        }
    }
}

impl Runtime {
    pub fn run(&mut self, program: Program) -> Result<Value, RuntimeError> {
        self.program = program;
        self.line = 0;
        self.index = 0;

        while self.line < self.program.lines() {
            // Immediate update, probably slower but who cares
            while self.index < self.program.get_line(self.line).len() {
                match &self.program.get_line(self.line)[self.index] {
                    Instruction::Dumpling(dumpling) => runtime_interpret_dumpling(dumpling, &mut self.line, &mut self.index, &mut self.stack, &mut self.natives)?,
                    Instruction::Other(operation) => runtime_interpret_other(operation, &mut self.line, &mut self.index, &mut self.stack, &mut self.natives)?,
                    _ => panic!("HOW?"),
                }
            }

            self.line += 1;
            self.index = 0; // silly me forgot this
        }

        if self.stack.len() > 0 {
            Ok(self.stack.pop().unwrap())
        } else {
            Ok(Value::Nil)
        }
    }
}

pub fn pop_stack(stack: &mut Vec<Value>) -> Result<Value, RuntimeError> {
    let Some(top) = stack.pop() else {
        return Err(RuntimeError::StackUnderflow);
    };
    Ok(top)
}

pub fn push_stack(stack: &mut Vec<Value>, value: Value) {
    stack.push(value);
}

// Since you won't let me nicely cut it up into methods and I saw someone using
// an "associated function", I'll just do this instead.
pub(crate) fn runtime_interpret_dumpling(dumpling: &Dumpling, line: &mut usize, index: &mut usize, stack: &mut Vec<Value>, natives: &mut HashMap<String, NativeFn>) -> Result<(), RuntimeError> {
    match dumpling {
        Dumpling::Float(val) => stack.push(Value::Float(*val)),
        Dumpling::FnCall(name) => {
            let Some(function) = natives.get(name) else {
                return Err(RuntimeError::NonexistentFunction(name.to_owned()));
            };

            let val = function(stack)?;
            stack.push(val);
        }
        Dumpling::Int(val) => stack.push(Value::Int(*val)),
        Dumpling::Jump => {
            let amount = pop_stack(stack)?;

            match amount {
                Value::Int(offset) => {
                    *line = offset as usize - 1; // line numbers start at 1 but indices start at 0
                    *index = 0;
                    return Ok(());
                },
                _ => return Err(RuntimeError::CustomError("cannot jump with a non-integer offset".to_string())),
            }
        },
        Dumpling::Stringify => {
            let top = pop_stack(stack)?;
            if let Value::RawText(top) = top {
                stack.push(Value::String(top));
            } else {
                stack.push(Value::String(top.to_string())); // work smarter, not harder
            }
        },
        Dumpling::Text(text) => stack.push(Value::RawText(text.clone())),
        _ => todo!("{:?}", dumpling),
    }
    *index += 1;

    Ok(())
}

pub(crate) fn runtime_interpret_other(operation: &Operation, line: &mut usize, index: &mut usize, stack: &mut Vec<Value>, natives: &mut HashMap<String, NativeFn>) -> Result<(), RuntimeError> {
    match operation {
        Operation::Eat => {
            let Some(top) = stack.pop() else {
                return Err(RuntimeError::StackUnderflow);
            };
            print!("{}", top);
    },
        _ => todo!(),
    }
    *index += 1;
    Ok(())
}
