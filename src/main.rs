
use std::io::{BufRead, Write};

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

    let mut stdin = std::io::stdin().lock();

    let terminate = false;

    while !terminate {
        let mut source: String = String::new();

        stdin.read_line(&mut source).unwrap();

        let span_tokens = dango_parser::span_tokenizer::tokenize_into_spans(source.as_str());
        let tokens = dango_parser::tokenizer::tokenize(span_tokens);

        let mut runtime = Runtime::new();

        dango_runtime::stdlib::load_io(&mut runtime);
        dango_runtime::stdlib::load_math(&mut runtime);

        runtime.run(dango_parser::parser::parse(tokens));

        println!();

        source.clear();
    }
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

    let span_tokens = dango_parser::span_tokenizer::tokenize_into_spans(source.as_str());
    let tokens = dango_parser::tokenizer::tokenize(span_tokens);

    let mut runtime = Runtime::new();

    dango_runtime::stdlib::load_io(&mut runtime);
    dango_runtime::stdlib::load_math(&mut runtime);

    runtime.run(dango_parser::parser::parse(tokens));
}
