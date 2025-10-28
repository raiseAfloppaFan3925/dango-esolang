
use std::io::Write;

use dango_parser;
use dango_runtime::{instructions::*, runtime::Runtime, Value};

static REPL_STRING: &str = "
 _|_
/@@@\\
\\@@@/  | Dango 0.1.0
/%%%\\  |
\\%%%/  | Stuff
/***\\  | Stuff
\\***/  | Stuff
  |
  |
";

fn repl() {
    println!("{}", REPL_STRING);

    todo!("REPL will be implemented after the parser is made");
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        repl();
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

    let span_tokens = dango_parser::tokenizer::tokenize(source.as_str());
    
    println!("{:?}", span_tokens);

    let mut runtime = Runtime::new();

    dango_runtime::stdlib::load_io(&mut runtime);
    dango_runtime::stdlib::load_math(&mut runtime);

    let mut program = Program::new();

    // eat (2.0)(:math-sqrt)----
    // program.add_line(vec![
    //     Instruction::Dumpling(Dumpling::Float(2.0)),
    //     Instruction::Dumpling(Dumpling::FnCall("math-sqrt".to_string())),
    //     Instruction::Other(Operation::Eat(Eat::StackTop)),
    // ]);

    // runtime.run(program);
}
