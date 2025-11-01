
use dango_errors::*;
use dango_runtime::{Value, instructions::Program, runtime::Runtime};

pub fn compile_str(source: &str) -> Result<Program, Vec<CompileError>> {
    let span_tokens = dango_parser::span_tokenizer::tokenize_into_spans(source);

    if let Err(errors) = span_tokens {
        return Err(errors);
    }

    let span_tokens = unsafe { span_tokens.unwrap_unchecked() };

    let res_tokens = dango_parser::tokenizer::tokenize(span_tokens);

    let Ok(tokens) = res_tokens else {
        return unsafe { Err(res_tokens.unwrap_err_unchecked()) };
    };

    dango_parser::parser::parse(tokens)
}

pub fn compile_string(source: String) -> Result<Program, Vec<CompileError>> {
    compile_str(source.as_str())
}

pub fn execute_str(runtime: &mut Runtime, source: &str) -> Result<Value, DangoError> {
    let program = compile_str(source);

    if let Ok(program) = program {
        let result = runtime.run(program);
        match result {
            Ok(value) => Ok(value),
            Err(err) => Err(DangoError::Runtime(err)),
        }
    } else {
        Err(DangoError::Compile(unsafe { program.unwrap_err_unchecked() }))
    }
}

pub fn execute_string(runtime: &mut Runtime, source: String) -> Result<Value, DangoError> {
    let program = compile_string(source);

    if let Ok(program) = program {
        let result = runtime.run(program);
        match result {
            Ok(value) => Ok(value),
            Err(err) => Err(DangoError::Runtime(err)),
        }
    } else {
        Err(DangoError::Compile(unsafe { program.unwrap_err_unchecked() }))
    }
}
