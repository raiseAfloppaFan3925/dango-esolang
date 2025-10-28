

pub mod instructions;
pub mod runtime;

use runtime::Runtime;

type NativeFn = fn(&mut Runtime) -> Value;

#[derive(Clone)]
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
