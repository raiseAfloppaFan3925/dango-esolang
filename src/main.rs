// This project turned from a dream to an annoyance

use std::io::Write;

use dango_core::*;
use dango_runtime::runtime::Runtime;

#[cfg(test)]
mod tests;

static DANGO_VERSION: &str = "0.11.0";

fn repl() -> std::io::Result<()> {
    let repl_string = format!("
   _|_
  \x1b[0;91m/@@@\\\x1b[0m  | Dango {}
  \x1b[0;91m\\@@@/\x1b[0m  |
  \x1b[0;93m/%%%\\\x1b[0m  | Documentation: https://raiseafloppafan3925.github.io/dango
  \x1b[0;93m\\%%%/\x1b[0m  | 'exit' to exit
  \x1b[0;92m/***\\\x1b[0m  |
  \x1b[0;92m\\***/\x1b[0m  | If you find any bugs, please report them at https://github.com/raiseAfloppaFan3925/dango/issues
    |    | We're back better than ever!
    |
", DANGO_VERSION);

    println!("{}", repl_string);

    loop {
        let mut source: String = String::new();

        print!("--\x1b[0;92m(O)\x1b[0;93m(O)\x1b[0;91m(O)\x1b[0m > ");
        let _ = std::io::stdout().flush();

        std::io::stdin().read_line(&mut source)?; // This is just the CLI for Dango so why make it (the CLI) multi-threaded?

        if source.trim() == "exit" {
            break;
        }

        let mut runtime = Runtime::new();

        dango_runtime::stdlib::load_io(&mut runtime);
        dango_runtime::stdlib::load_chrono(&mut runtime);
        dango_runtime::stdlib::load_math(&mut runtime);

        let value = dango_utils::execute_str(&mut runtime, source.as_str());

        // Prevents any `write(stdout)` or `eat` commands from appearing AFTER what's about to be printed below
        // Doesn't matter, this is a REPL
        std::io::stdout().flush()?;
        
        match value {
            Ok(value) => println!("\n{}", value),
            Err(err) => eprintln!("\n{}", err), // Don't exit, this is a REPL
        }

        source.clear();
    }

    std::process::exit(0);
}

fn main() -> std::io::Result<()> {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        repl()?;
        std::process::exit(0);
    }

    let path = args.get(1).unwrap();

    let Ok(source) = std::fs::read(path) else {
        eprintln!("Error: Could not read from path {path}");
        std::process::exit(1);
    };

    let Ok(source) = String::from_utf8(source) else {
        eprintln!("Error: file contains invalid Unicode");
        std::process::exit(1);
    };

    let mut runtime = Runtime::new();

    dango_runtime::stdlib::load_io(&mut runtime);
    dango_runtime::stdlib::load_math(&mut runtime);
    dango_runtime::stdlib::load_chrono(&mut runtime);

    let value = dango_utils::execute_str(&mut runtime, source.as_str());

    if let Err(err) = value {
        println!("\n{}", err);
        std::process::exit(1);
    }

    Ok(())
}
