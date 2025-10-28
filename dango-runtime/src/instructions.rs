
/// The fundamental unit of code in the dango in Dango
/// 
/// ```text
/// (Hello, world!)(')----
/// eat
/// ```
#[derive(Debug)]
pub enum Dumpling {
    Float(f64),                   // (0.401)
    FnCall(String),               // (:get-args)
    Int(i64),                     // (39)
    Jump,                         // (420)(j)
    Left,                         // (%left)
    Null,                         // ()
    Text(String),                 // (Hello, world!)
    Stringify,                    // (')
    StringifyRawUtf32(i64),       // (N)('b)
}

#[derive(Debug)]
pub enum Eat {
    Index(usize),                 // eat 39
    StackTop,                     // eat
}

#[derive(Debug)]
pub enum Operation {
    Eat(Eat),
}

#[derive(Debug)]
pub enum Instruction {
    Dumpling(Dumpling),
    Other(Operation),
}

#[derive(Debug)]
pub struct Program {
    code: Vec<Vec<Instruction>>,
}

impl Program {
    pub fn new() -> Self {
        Self {
            code: vec![],
        }
    }

    pub fn get_line(&self, line: usize) -> &Vec<Instruction> {
        &self.code[line]
    }

    pub fn get_line_mut(&mut self, line: usize) -> &mut Vec<Instruction> {
        &mut self.code[line]
    }

    pub fn add_line(&mut self, line: Vec<Instruction>) {
        self.code.push(line)
    }

    pub fn lines(&self) -> usize {
        self.code.len()
    }
}
