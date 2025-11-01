

pub mod instructions;
pub mod runtime;

#[cfg(not(feature = "exclude-stdlib"))]
pub mod stdlib;

use std::{collections::HashMap, io::Write};

use dango_errors::RuntimeError;

use instructions::{Instruction, Program};
use runtime::Runtime;

type NativeFn = fn(&mut Vec<Value>) -> Result<Value, RuntimeError>;

#[derive(Debug, Clone)]
pub enum Value {
    Nil,                    // nil, but ()
    Int(i64),               // i64
    Float(f64),             // f64
    String(String),         // stringified value
    RawText(String),        // raw text
    Dango(Vec<Value>),      // array-like
}

impl Value {
    pub fn dango_from_vec(mut vec: Vec<Value>) -> Self {
        if vec.len() < 5 {
            return Self::Dango(vec);
        }

        let mut tmp = vec![];
        vec.reverse();

        for item in vec {
            if tmp.len() == 5 {
                tmp = vec![Value::Dango(tmp)];
            }

            tmp.insert(0, item);
        }

        Value::Dango(tmp)
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
                (Self::Nil, Self::Nil) => true,

                (Self::Int(a), Self::Int(b)) => *a == *b,
                (Self::Int(a), Self::Float(b)) => *a as f64 == *b,
                (Self::Float(a), Self::Int(b)) => *a == *b as f64,
                (Self::Float(a), Self::Float(b)) => *a == *b,

                (Self::String(a), Self::String(b)) => a == b,

                (Self::Dango(a), Self::Dango(b)) => {
                    // Element-wise equality
                    if a.len() != b.len() { return false; }

                    for i in 0..a.len() {
                        if a.get(i) != b.get(i) {
                            return false;
                        }
                    }

                    true
                }

                _ => false,
            }
    }

    fn ne(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => false,

            (Self::Int(a), Self::Int(b)) => *a != *b,
            (Self::Int(a), Self::Float(b)) => *a as f64 != *b,
            (Self::Float(a), Self::Int(b)) => *a != *b as f64,
            (Self::Float(a), Self::Float(b)) => *a != *b,

            (Self::String(a), Self::String(b)) => a != b,

            (Self::Dango(a), Self::Dango(b)) => {
                if a.len() != b.len() { return false; }
                
                for i in 0..a.len() {
                    if a.get(i) != b.get(i) { return true; }
                }
                
                false
            }

            _ => true,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
            (Value::Int(a), Value::Float(b)) => (*a as f64).partial_cmp(b),
            (Value::Float(a), Value::Int(b)) => a.partial_cmp(&(*b as f64)),
            (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),

            (Value::Dango(a), Value::Dango(b)) => a.len().partial_cmp(&b.len()),

            _ => None,
        }
    }
}

#[allow(unreachable_patterns)]
impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "()"),
            Self::Int(val) => write!(f, "{}", val),
            Self::Float(val) => write!(f, "{}", val),
            Self::String(val) => write!(f, "{}", val),
            Self::RawText(val) => write!(f, "({})", val),
            Self::Dango(values) => {
                for value in values {
                    if let Value::Dango(_) = value {
                        let mut tmp = format!("[{}", value);
                        tmp.truncate(tmp.len() - 4);

                        write!(f, "{}]", tmp)?;
                    } else {
                        write!(f, "({})", value)?;
                    }
                }

                write!(f, "----")
            }
            _ => todo!(),
        }
    }
}

impl Runtime {
    pub fn run(&mut self, program: Program) -> Result<Value, RuntimeError> {
        self.program = program;
        self.line = 0;
        self.index = 0;
        self.in_while = false;

        while self.line < self.program.lines() {
            // Immediate update, probably slower but who cares
            while self.index < self.program.get_line(self.line).len() {
                runtime_interpret(&self.program.get_line(self.line)[self.index], &mut self.line, &mut self.index, &mut self.in_while, &mut self.stack, &mut self.natives)?;
            }

            if !self.in_while {
                self.line += 1;
            }
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

pub fn peek_stack(stack: &mut Vec<Value>, depth: usize) -> Result<&Value, RuntimeError> {
    if stack.len() <= depth {
        return Err(RuntimeError::StackUnderflow);
    }

    Ok(unsafe { stack.get(stack.len() - depth - 1).unwrap_unchecked() }) // the if above prevents invalid indices unless someone tells me that it doesn't
}

// Ugly code, but I demand integers and zero division.
#[inline]
fn safe_division_int(a: i64, b: i64) -> Result<i64, f64> {
    if b == 0 {
        if a == 0 {
            return Err(f64::NAN);
        } else {
            return Err(f64::INFINITY);
        }
    }

    Ok(a / b)
}

// Since you won't let me nicely cut it up into methods and I saw someone using
// an "associated function", I'll just do this instead.
pub(crate) fn runtime_interpret(instruction: &Instruction, line: &mut usize, index: &mut usize, in_while: &mut bool, stack: &mut Vec<Value>, natives: &mut HashMap<String, NativeFn>) -> Result<(), RuntimeError> {
    match instruction {
        Instruction::Add => {
            let b = pop_stack(stack)?;
            let a = pop_stack(stack)?;

            match (a, b) {
                // Numeric types
                (Value::Int(a), Value::Int(b)) => push_stack(stack, Value::Int(a.overflowing_add(b).0)),
                (Value::Int(a), Value::Float(b)) => push_stack(stack, Value::Float(a as f64 + b)),
                (Value::Float(a), Value::Int(b)) => push_stack(stack, Value::Float(a + b as f64)),
                (Value::Float(a), Value::Float(b)) => push_stack(stack, Value::Float(a + b)),
                // String concatenation
                (Value::String(a), Value::String(b)) => {
                    let mut tmp = a;
                    tmp.push_str(b.as_str());
                    push_stack(stack, Value::String(tmp));
                }
                // Dango concatenation
                (Value::Dango(mut a), Value::Dango(mut b)) => {
                    if a.len() + b.len() > 5 {
                        return Err(RuntimeError::CustomError(format!("cannot create new dango as the lengths ({} and {}) combined are too long", a.len(), b.len())));
                    }

                    a.append(&mut b); // The value has been popped anyway

                    push_stack(stack, Value::Dango(a));
                }
                _ => return Err(RuntimeError::IncorrectOperationTypes("addition".to_string())),
            }
        }
        Instruction::CharFromCodePoint => {
            let value = pop_stack(stack)?;

            if let Value::Int(cp) = value {
                if cp < 0 {
                    return Err(RuntimeError::NotACodePoint(Some(cp)));
                }

                let Some(character) = char::from_u32(cp as u32) else {
                    return Err(RuntimeError::NotACodePoint(Some(cp)));
                };

                push_stack(stack, Value::String(String::from(character)));
            } else {
                return Err(RuntimeError::NotACodePoint(None));
            }
        }
        Instruction::Divide => {
            let b = pop_stack(stack)?;
            let a = pop_stack(stack)?;

            match (a, b) {
                // Numeric types
                (Value::Int(a), Value::Int(b)) => {
                    let val = safe_division_int(a, b);
                    if let Ok(as_int) = val {
                        push_stack(stack, Value::Int(as_int));
                    } else {
                        // I wish Rust provided better syntax for `else` in `if let` where there's only two cases
                        let as_float = unsafe { val.unwrap_err_unchecked() };
                        push_stack(stack, Value::Float(as_float));
                    }
                }
                (Value::Int(a), Value::Float(b)) => push_stack(stack, Value::Float(a as f64 / b)),
                (Value::Float(a), Value::Int(b)) => push_stack(stack, Value::Float(a / b as f64)),
                (Value::Float(a), Value::Float(b)) => push_stack(stack, Value::Float(a / b)),
                _ => return Err(RuntimeError::IncorrectOperationTypes("division".to_string())),
            }
        }
        Instruction::Equal => {
            let b = pop_stack(stack)?;
            let a = pop_stack(stack)?;

            push_stack(stack, Value::Int((a == b) as i64));
        }
        Instruction::Float(val) => stack.push(Value::Float(*val)),
        Instruction::FnCall(name) => {
            let Some(function) = natives.get(name) else {
                return Err(RuntimeError::NonexistentFunction(name.to_owned()));
            };

            let val = function(stack)?;
            stack.push(val);
        }
        Instruction::Greater => {
            let b = pop_stack(stack)?;
            let a = pop_stack(stack)?;

            push_stack(stack, Value::Int((a > b) as i64));
        }
        Instruction::Int(val) => stack.push(Value::Int(*val)),
        Instruction::Jump => {
            let amount = pop_stack(stack)?;

            *in_while = false;

            match amount {
                Value::Int(offset) => {
                    *line = offset as usize - 1; // line numbers start at 1 but indices start at 0
                    *index = 0;
                    return Ok(());
                }
                _ => return Err(RuntimeError::CustomError("cannot jump with a non-integer offset".to_string())),
            }
        }
        Instruction::Length => {
            let Ok(top) = peek_stack(stack, 0) else {
                push_stack(stack, Value::Int(0));
                return Ok(());
            };

            let mut len = 0;

            if let Value::Dango(dango) = top {
                len = dango.len();
            }
            
            push_stack(stack, Value::Int(len as i64));
        }
        Instruction::Less => {
            let b = pop_stack(stack)?;
            let a = pop_stack(stack)?;

            push_stack(stack, Value::Int((a < b) as i64));
        }
        Instruction::Multiply => {
            let b = pop_stack(stack)?;
            let a = pop_stack(stack)?;

            match (a, b) {
                // Numeric types
                (Value::Int(a), Value::Int(b)) => push_stack(stack, Value::Int(a.overflowing_mul(b).0)),
                (Value::Int(a), Value::Float(b)) => push_stack(stack, Value::Float(a as f64 * b)),
                (Value::Float(a), Value::Int(b)) => push_stack(stack, Value::Float(a * b as f64)),
                (Value::Float(a), Value::Float(b)) => push_stack(stack, Value::Float(a * b)),
                _ => return Err(RuntimeError::IncorrectOperationTypes("multiplication".to_string())),
            }
        }
        Instruction::Null => push_stack(stack, Value::Nil),
        Instruction::NotEqual => {
            let b = pop_stack(stack)?;
            let a = pop_stack(stack)?;

            push_stack(stack, Value::Int((a != b) as i64));
        }
        Instruction::Stringify => {
            let top = pop_stack(stack)?;
            if let Value::RawText(top) = top {
                stack.push(Value::String(top));
            } else {
                stack.push(Value::String(top.to_string())); // work smarter, not harder
            }
        }
        Instruction::Subtract => {
            let b = pop_stack(stack)?;
            let a = pop_stack(stack)?;

            match (a, b) {
                // Numeric types
                (Value::Int(a), Value::Int(b)) => push_stack(stack, Value::Int(a.overflowing_sub(b).0)),
                (Value::Int(a), Value::Float(b)) => push_stack(stack, Value::Float(a as f64 - b)),
                (Value::Float(a), Value::Int(b)) => push_stack(stack, Value::Float(a - b as f64)),
                (Value::Float(a), Value::Float(b)) => push_stack(stack, Value::Float(a - b)),
                _ => return Err(RuntimeError::IncorrectOperationTypes("subtraction".to_string())),
            }
        }
        Instruction::Text(text) => stack.push(Value::RawText(text.clone())),
        Instruction::Eat => print!("{}", pop_stack(stack)?),
        Instruction::Fetch(depth) => {
            let depth = *depth;

            let value = peek_stack(stack, depth)?.clone();
            push_stack(stack, value);
        }
        Instruction::Remove => {
            let top = pop_stack(stack)?;

            if let Value::Dango(mut dango) = top {
                // Zero-length dango cannot exist, but if you make one then that's YOUR fault, not mine
                dango.reverse();
                let value = unsafe { dango.pop().unwrap_unchecked() };
                dango.reverse();

                if dango.len() > 0 {
                    push_stack(stack, Value::Dango(dango));
                }
                push_stack(stack, value);
            }
        }
        Instruction::Skewer(count) => {
            let count = *count;

            if count == 0 {
                return Err(RuntimeError::CustomError("cannot have dango with zero dumplings, that's just a stick".to_string()));
            }

            let mut values: Vec<Value> = vec![];

            for _ in 0..count {
                values.push(pop_stack(stack)?);
            }

            if count > 5 {
                return Err(RuntimeError::CustomError(format!("skewer is too short for {} dumplings", count)));
            }

            stack.push(Value::Dango(values));
        }
        Instruction::While => {
            let condition = pop_stack(stack)?;

            *in_while = match condition {
                Value::Nil => false,
                Value::Int(val) => val != 0,
                Value::Float(val) => val != 0.0,
                Value::String(string) => string.len() > 0,
                Value::Dango(dango) => dango.len() > 0,
                _ => false,
            };

            if !*in_while {
                *index = 0;
                *line += 1;
            }
        }
        Instruction::Nop => (),
    }
    *index += 1;
    Ok(())
}
