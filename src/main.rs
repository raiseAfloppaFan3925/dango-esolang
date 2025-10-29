
use std::io::{BufRead, Write};

use dango_runtime::runtime::Runtime;

static REPL_STRING: &str = "
 _|_
/@@@\\  | Dango 0.1.0
\\@@@/  |
/%%%\\  | Documentation: does not exist yet lol
\\%%%/  | 'exit' to exit
/***\\  |
\\***/  | If you see any bugs, please report them at https://github.com/raiseAfloppaFan3925/dango-esolang/issues
  |    |
  |
";

fn repl() {
    println!("{}", REPL_STRING);

    let mut stdout = std::io::stdout().lock();
    let mut stdin = std::io::stdin().lock();

    let terminate = false;

    while !terminate {
        let mut source: String = String::new();

        print!("--(O)(O)(O) > ");
        let _ = stdout.flush();

        stdin.read_line(&mut source).unwrap();

        if source.trim() == "exit" {
            std::process::exit(0);
        }

        let mut runtime = Runtime::new();

        dango_runtime::stdlib::load_io(&mut runtime);
        dango_runtime::stdlib::load_math(&mut runtime);

        let value = dango_utils::execute_str(&mut runtime, source.as_str());
        
        match value {
            Ok(value) => println!("\n{}", value),
            Err(err) => {
                eprintln!("Error: {}", err);
                std::process::exit(1);
            }
        }

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

    let mut runtime = Runtime::new();

    dango_runtime::stdlib::load_io(&mut runtime);
    dango_runtime::stdlib::load_math(&mut runtime);

    let value = dango_utils::execute_str(&mut runtime, source.as_str());

    if let Err(err) = value {
        println!("Error: {}", err);
        std::process::exit(1);
    }
}
