
#[derive(Debug, Clone)]
pub enum Instruction {
    Add,                          // (+)
    CharFromCodePoint,            // ('c)
    Divide,                       // (/)
    Equal,                        // (=)
    Float(f64),                   // (0.401)
    FnCall(String),               // (:get-args)
    Greater,                      // (>)
    Int(i64),                     // (39)
    Jump,                         // (420)(j)
    Less,                         // (<)
    Length,                       // (len)
    Multiply,                     // (*)
    Nop,                          // needed for the interpreter to not crash out
    NotEqual,                     // (!=)
    Null,                         // ()
    Text(String),                 // (Hello, world!)
    Stringify,                    // (')
    Subtract,                     // (-)
    While,

    Eat,
    Fetch(usize),
    Remove,
    Skewer(u8),
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
        self.code.push(line);
    }

    pub fn lines(&self) -> usize {
        self.code.len()
    }
}
