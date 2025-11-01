
use super::*;

use dango_runtime::{Value, runtime::Runtime};

#[test]
fn test_addition() {
    let mut runtime = Runtime::new();

    let result = dango_utils::execute_str(&mut runtime, "(+)(1)(2)----");
    assert_eq!(result, Ok(Value::Int(3)));

    let result = dango_utils::execute_str(&mut runtime, "(+)(1.0)(2.0)----");
    assert_eq!(result, Ok(Value::Float(3.0)));

    let result = dango_utils::execute_str(&mut runtime, "(+)(')(world!)(')(Hello, )----");
    assert_eq!(result, Ok(Value::String("Hello, world!".to_string())));
}

#[test]
fn test_comparison() {
    let mut runtime = Runtime::new();

    let result = dango_utils::execute_str(&mut runtime, "(>)(1)(2)----");
    assert_eq!(result, Ok(Value::Int(1)));

    let result = dango_utils::execute_str(&mut runtime, "(<)(1)(2)----");
    assert_eq!(result, Ok(Value::Int(0)));

    let result = dango_utils::execute_str(&mut runtime, "(>)(2)(1)----");
    assert_eq!(result, Ok(Value::Int(0)));

    let result = dango_utils::execute_str(&mut runtime, "(<)(2)(1)----");
    assert_eq!(result, Ok(Value::Int(1)));
}

#[test]
fn test_dango_data_structure() {
    let mut runtime = Runtime::new();
    
    let result = dango_utils::execute_str(&mut runtime, "skewer 2 (1)(2)----");
    assert_eq!(result, Ok(Value::Dango(vec![
        Value::Int(1),
        Value::Int(2)
    ])));
}

#[test]
fn test_compile_errors() {
    let result = dango_utils::compile_str("source");
    assert!(result.is_err());

    let result = dango_utils::compile_str("----");
    assert!(result.is_err());

    let result = dango_utils::compile_str("eat ;: Hello\n:; (2)----");
    assert!(result.is_err());
}
