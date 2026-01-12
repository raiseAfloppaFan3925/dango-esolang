
pub mod instructions;
pub mod runtime;

#[cfg(not(feature = "exclude-stdlib"))]
pub mod stdlib;

use std::io::Write;

use dango_errors::RuntimeError;

use instructions::{Instruction, Program};
use runtime::Runtime;

type NativeFn = fn(&mut Runtime) -> Result<Value, RuntimeError>;

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
        self.in_while = false;

        while self.line < self.program.lines() {
            self.index = 0;
            while self.index < self.program.get_line(self.line).len() {
                // you better optimize this `clone` call.
                self.run_inst(self.program.get_line(self.line)[self.index].clone())?;
            }

            if !self.in_while {
                self.line += 1;
            }
        }

        if self.stack.len() > 0 {
            Ok(self.stack.pop().unwrap())
        } else {
            Ok(Value::Nil)
        }
    }
    
    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Result<Value, RuntimeError> {
        match self.stack.pop() {
            Some(value) => Ok(value),
            None => Err(RuntimeError::StackUnderflow),
        }
    }

    pub fn peek(&self, depth: usize) -> Result<&Value, RuntimeError> {
        match self.stack.get(self.stack.len() - depth - 1) {
            Some(value) => Ok(value),
            None => Err(RuntimeError::StackUnderflow),
        }
    }

    pub fn dump_stack(&self) {
        println!("---- stack ----");
        let mut i = 0;
        for value in &self.stack {
            println!("{:#06x} {}", i, value);
            i += 1;
        }
        println!("---- stack end ----");
    }

    fn run_inst(&mut self, instruction: Instruction) -> Result<(), RuntimeError> {
        self.index += 1;
        match instruction {
            Instruction::Add => {
                let b = self.pop()?;
                let a = self.pop()?;

                match (a, b) {
                    (Value::Int(a), Value::Int(b)) => self.stack.push(Value::Int(a.wrapping_add(b))),
                    (Value::Int(a), Value::Float(b)) => self.stack.push(Value::Float(a as f64 + b)),
                    (Value::Float(a), Value::Int(b)) => self.stack.push(Value::Float(a + b as f64)),
                    (Value::Float(a), Value::Float(b)) => self.stack.push(Value::Float(a + b)),
                    (Value::String(a), Value::String(b)) => self.stack.push(Value::String(a + b.as_str())),
                    (Value::Dango(a), Value::Dango(b)) => {
                        if a.len() + b.len() > 5 {
                            return Err(RuntimeError::CustomError(format!("cannot create new dango as the lengths ({} and {}) combined are too long", a.len(), b.len())));
                        }
                        let mut vector = a;
                        vector.extend(b);
                        self.stack.push(Value::dango_from_vec(vector));
                    }
                    _ => return Err(RuntimeError::IncorrectOperationTypes("addition".to_string())),
                }
            }
            Instruction::CharFromCodePoint => match self.pop()? {
                Value::Int(codepoint) => match u32::try_from(codepoint) {
                    Ok(codepoint) => match char::from_u32(codepoint) {
                        Some(character) => self.stack.push(Value::String(character.to_string())),
                        None => return Err(RuntimeError::NotACodePoint(Some(codepoint.into()))),
                    }
                    Err(_) => return Err(RuntimeError::NotACodePoint(Some(codepoint))),
                }
                _ => return Err(RuntimeError::NotACodePoint(None)),
            }
            Instruction::Divide => {
                let b = self.pop()?;
                let a = self.pop()?;

                match (a, b) {
                    (Value::Int(a), Value::Int(b)) => {
                        let val = safe_division_int(a, b);
                        // I wish Rust provided better syntax for `else` in `if let` where there's only two cases
                        // well buddy, have you ever heard of "match"?
                        match val {
                            Ok(as_int) => self.stack.push(Value::Int(as_int)),
                            Err(as_float) => self.stack.push(Value::Float(as_float)),
                        }
                    }
                    _ => return Err(RuntimeError::IncorrectOperationTypes("division".to_string())),
                }
            }
            Instruction::Eat => {
                print!("{}", self.pop()?);
                // Unlike in the CLI, we do want to lock `stdout` here because someone could be using Dango
                // in a multi-threaded context.
                std::io::stdout().lock().flush().unwrap();
            }
            Instruction::Equal => {
                let b = self.pop()?;
                let a = self.pop()?;

                self.stack.push(Value::Int((a == b).into()));
            }
            Instruction::Fetch(depth) => self.stack.push(self.peek(depth)?.clone()),
            Instruction::Float(val) => self.stack.push(Value::Float(val)),
            Instruction::FnCall(name) => {
                let Some(function) = self.natives.get(&name) else {
                    return Err(RuntimeError::NonexistentFunction(name.to_owned()));
                };

                let val = function(self)?;
                self.stack.push(val);
            }
            Instruction::Greater => {
                let b = self.pop()?;
                let a = self.pop()?;

                self.stack.push(Value::Int((a > b).into()));
            }
            Instruction::Int(value) => self.stack.push(Value::Int(value)),
            Instruction::Jump => {
                let offset = self.pop()?;

                self.in_while = false;

                match offset {
                    Value::Int(offset) => {
                        self.line = offset as usize - 1; // line numbers start at 1 but indices start at 0
                        self.index = 0;
                        return Ok(());
                    }
                    _ => return Err(RuntimeError::CustomError("cannot jump with a non-integer offset".to_string())),
                }
            }
            // BREAKING CHANGE: `(len)` on an empty stack now throws a stack overflow error instead
            //                  of returning zero
            Instruction::Length => match self.peek(0)? {
                Value::Dango(dango) => self.push(Value::Int(dango.len() as i64)),
                _ => self.push(Value::Int(0)),
            }
            Instruction::Less => {
                let b = self.pop()?;
                let a = self.pop()?;

                self.stack.push(Value::Int((a < b).into()));
            }
            Instruction::Multiply => {
                let b = self.pop()?;
                let a = self.pop()?;

                match (a, b) {
                    (Value::Int(a), Value::Int(b)) => self.stack.push(Value::Int(a.wrapping_mul(b))),
                    (Value::Int(a), Value::Float(b)) => self.stack.push(Value::Float(a as f64 * b)),
                    (Value::Float(a), Value::Int(b)) => self.stack.push(Value::Float(a * b as f64)),
                    (Value::Float(a), Value::Float(b)) => self.stack.push(Value::Float(a * b)),
                    (Value::String(a), Value::Int(b)) => match b {
                        i64::MIN..0 => return Err(RuntimeError::CustomError("cannot copy a string a negative number of times".to_string())),
                        0 => self.stack.push(Value::String(String::new())),
                        1 => self.stack.push(Value::String(a)),
                        _ => {
                            let mut value = a.clone();
                            for _ in 1..=b {
                                value.push_str(a.as_str());
                            }
                            self.stack.push(Value::String(value));
                        }
                    }
                    _ => return Err(RuntimeError::IncorrectOperationTypes("multiplication".to_string())),
                }
            }
            Instruction::Nop => {}
            Instruction::NotEqual => {
                let b = self.pop()?;
                let a = self.pop()?;

                self.stack.push(Value::Int((a != b).into()));
            }
            Instruction::Null => self.stack.push(Value::Nil),
            Instruction::Remove => match self.pop()? {
                Value::Dango(mut dango) => {
                    
                    // Zero-length dango CAN exist and even if it's your fault, I don't want
                    // the program to bring down your program with UB just because you did it.
                    dango.reverse();
                    let value = dango.pop().ok_or(RuntimeError::ZeroLengthDango)?;
                    dango.reverse();

                    if dango.len() > 0 {
                        self.stack.push(Value::Dango(dango));
                    }
                    self.stack.push(value);
                }
                _ => (),
            }
            Instruction::Skewer(count) => match count {
                0 => return Err(RuntimeError::CustomError("cannot have dango with zero dumplings, that's just a stick".to_string())),
                1..=5 => {
                    let mut values = vec![];

                    for _ in 0..count {
                        values.push(self.pop()?);
                    }

                    self.stack.push(Value::Dango(values));
                }
                _ => return Err(RuntimeError::CustomError(format!("skewer is too short for {} dumplings", count))),
            }
            Instruction::Subtract => {
                let b = self.pop()?;
                let a = self.pop()?;

                match (a, b) {
                    // Numeric types
                    (Value::Int(a), Value::Int(b)) => self.stack.push(Value::Int(a.wrapping_sub(b))),
                    (Value::Int(a), Value::Float(b)) => self.stack.push(Value::Float(a as f64 - b)),
                    (Value::Float(a), Value::Int(b)) => self.stack.push(Value::Float(a - b as f64)),
                    (Value::Float(a), Value::Float(b)) => self.stack.push(Value::Float(a - b)),
                    _ => return Err(RuntimeError::IncorrectOperationTypes("subtraction".to_string())),
                }
            }
            Instruction::Stringify => match self.pop()? {
                Value::RawText(text) => self.stack.push(Value::String(text)),
                value => self.stack.push(Value::String(value.to_string())),
            }
            Instruction::Text(text) => self.stack.push(Value::RawText(text)),
            Instruction::ToFloat => match self.pop()? {
                Value::Int(x) => self.push(Value::Float(x as f64)),
                Value::Float(x) => self.push(Value::Float(x)),
                Value::String(string) => match string.parse::<f64>() {
                    Ok(value) => self.push(Value::Float(value)),
                    Err(_) => self.push(Value::Nil),
                }
                _ => self.push(Value::Nil),
            }
            Instruction::ToInt => match self.pop()? {
                Value::Int(x) => self.push(Value::Int(x)),
                Value::Float(x) => self.push(Value::Int(x as i64)),
                Value::String(string) => match string.parse::<i64>() {
                    Ok(value) => self.push(Value::Int(value)),
                    Err(_) => self.push(Value::Nil),
                }
                _ => self.push(Value::Nil),
            }
            Instruction::While => {
                let condition = self.pop()?;

                self.in_while = match condition {
                    Value::Nil => false,
                    Value::Int(val) => val != 0,
                    Value::Float(val) => val != 0.0,
                    Value::String(string) => string.len() > 0,
                    Value::Dango(dango) => dango.len() > 0,
                    _ => false,
                };

                if !self.in_while {
                    self.index = 0;
                    self.line += 1;
                }
            }
        }
        Ok(())
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

    Ok(a.wrapping_div(b))
}

// Good riddance, `runtime_interpret`!
