
use dango_runtime::{RuntimeError, Value, instructions::Program, runtime::Runtime};

pub fn compile_str(source: &str) -> Program {
    let span_tokens = dango_parser::span_tokenizer::tokenize_into_spans(source);
    let tokens = dango_parser::tokenizer::tokenize(span_tokens);
    dango_parser::parser::parse(tokens)
}

pub fn compile_string(source: String) -> Program {
    let span_tokens = dango_parser::span_tokenizer::tokenize_into_spans(source.as_str());
    let tokens = dango_parser::tokenizer::tokenize(span_tokens);
    dango_parser::parser::parse(tokens)
}

pub fn execute_str(runtime: &mut Runtime, source: &str) -> Result<Value, RuntimeError> {
    let program = compile_str(source);
    runtime.run(program)
}

pub fn execute_string(runtime: &mut Runtime, source: String) -> Result<Value, RuntimeError> {
    let program = compile_string(source);
    runtime.run(program)
}
