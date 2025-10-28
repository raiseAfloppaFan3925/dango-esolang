
use std::io::Write;

use dango_runtime::{instructions::*, runtime::Runtime, Value};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        let mut stderr = std::io::stderr().lock();
        let _ = writeln!(stderr, "Usage: dango [file.dango]");
        std::process::exit(1);
    }

    let path = args.get(1).unwrap();
    let Ok(source) = std::fs::read(path) else {
        let mut stderr = std::io::stderr().lock();
        let _ = writeln!(stderr, "Error: Could not read from path {path}");
        std::process::exit(1);
    };
    let Ok(source) = String::from_utf8(source) else {
        let mut stderr = std::io::stderr().lock();
        let _ = writeln!(stderr, "Error: file contains invalid Unicode");
        std::process::exit(1);
    };
    
    println!("{}", source);

    let mut runtime = Runtime::new(1024);

    runtime.register_function("square".to_string(), |rt| {
        match rt.pop_stack() {
            Value::Int(val) => Value::Int(val * val),
            Value::Float(val) => Value::Float(val * val),
            _ => Value::Nil,
        }
    });

    let mut program = Program::new();

    // eat (Hello, world!)(')----
    // this implies that programs are parsed in reverse LOL
    program.add_line(vec![
        Instruction::Dumpling(Dumpling::Text("Hello, world!".to_string())),
        Instruction::Dumpling(Dumpling::Stringify),
        Instruction::Other(Operation::Eat(Eat::StackTop)),
    ]);

    runtime.run(program);
}
