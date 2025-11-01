
use std::collections::HashMap;

use super::{NativeFn, Value};
use super::instructions::*;

pub struct Runtime {
    pub(crate) program: Program,
    pub(crate) line: usize,
    pub(crate) index: usize,

    pub(crate) in_while: bool,

    pub(crate) natives: HashMap<String, NativeFn>,
    pub(crate) stack: Vec<Value>,
}

impl Runtime {
    pub fn new() -> Self {
        Self {
            program: Program::new(),
            line: 0,
            index: 0,

            in_while: false,

            natives: HashMap::new(),
            stack: vec![],
        }
    }

    pub fn register_function(&mut self, name: String, func: NativeFn) {
        self.natives.insert(name, func);
    }
}
