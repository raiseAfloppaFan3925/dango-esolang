//! 🍡Dango's standard library. This is a separate module since you might not want to load it.
use super::*;

pub fn load_io(runtime: &mut Runtime) {
    runtime.register_function("io-input".to_string(), dango_io_input);
    runtime.register_function("io-write".to_string(), dango_io_write);

    runtime.register_function("env-args".to_string(), dango_env_args);
}

pub fn load_math(runtime: &mut Runtime) {
    runtime.register_function("math-abs".to_string(), dango_math_abs);
    runtime.register_function("math-asin".to_string(), dango_math_asin);
    runtime.register_function("math-acos".to_string(), dango_math_acos);
    runtime.register_function("math-atan".to_string(), dango_math_atan);
    runtime.register_function("math-cos".to_string(), dango_math_cos);
    runtime.register_function("math-logb".to_string(), dango_math_logb);
    runtime.register_function("math-pi".to_string(), dango_math_pi);
    runtime.register_function("math-pow".to_string(), dango_math_pow);
    runtime.register_function("math-sin".to_string(), dango_math_sin);
    runtime.register_function("math-sqrt".to_string(), dango_math_sqrt);
    runtime.register_function("math-sqrt2".to_string(), dango_math_sqrt2);
    runtime.register_function("math-tan".to_string(), dango_math_tan);
}

fn dango_io_input(stack: &mut Vec<Value>) -> Result<Value, RuntimeError> {
    use std::io;
    use std::io::BufRead;

    let mode = pop_stack(stack)?;
    
    if let Value::Int(mode) = mode {
        match mode {
            0 => (),
            1 => {
                let prompt = pop_stack(stack)?;
                print!("{}", prompt);
            },
            _ => push_stack(stack, Value::Int(mode)),
        }
    } else {
        push_stack(stack, mode);
    }

    let mut stdin = io::stdin().lock();

    let mut read_value = String::new();
    let Ok(_) = stdin.read_line(&mut read_value) else {
        return Err(RuntimeError::CustomError("`:io-input` internal error: failed to read from input stream".to_string()));
    };

    Ok(Value::String(read_value))
}

fn dango_io_write(stack: &mut Vec<Value>) -> Result<Value, RuntimeError> {
    let Value::String(target) = pop_stack(stack)? else {
        return Err(RuntimeError::CustomError("`:io-write` error: target stream value is a string".to_string()));
    };

    let value = pop_stack(stack)?;

    match target.as_str() {
        "stdin" => return Err(RuntimeError::CustomError("`:io-write` error: cannot write to stdin".to_string())),
        "stdout" => {
            let mut stdout = std::io::stdout().lock();
            let _ = write!(stdout, "{}", value);
            let _ = stdout.flush();
        },
        "stderr" => {
            let mut stderr = std::io::stderr().lock();
            let _ = write!(stderr, "{}", value);
            let _ = stderr.flush();
        },
        _ => {
            let _ = std::fs::write(target, value.to_string());
        },
    }

    Ok(Value::Nil)
}

fn dango_math_abs(stack: &mut Vec<Value>) -> Result<Value, RuntimeError> {
    let val = pop_stack(stack)?;

    Ok(match val {
        Value::Int(val) => Value::Int(val.abs()),
        Value::Float(val) => Value::Float(val.abs()),
        _ => return Err(RuntimeError::CustomError("`:math-sqrt` error: value is invalid".to_string())),
    })
}

fn dango_math_asin(stack: &mut Vec<Value>) -> Result<Value, RuntimeError> {
    let val = pop_stack(stack)?;

    Ok(Value::Float(match val {
        Value::Int(val) => (val as f64).asin(),
        Value::Float(val) => val.asin(),
        _ => return Err(RuntimeError::CustomError("`:math-asin` error: value is invalid".to_string())),
    }))
}

fn dango_math_acos(stack: &mut Vec<Value>) -> Result<Value, RuntimeError> {
    let val = pop_stack(stack)?;

    Ok(Value::Float(match val {
        Value::Int(val) => (val as f64).acos(),
        Value::Float(val) => val.acos(),
        _ => panic!("dango stdlib `math-acos` error: value {val} is invalid"),
    }))
}

fn dango_math_atan(stack: &mut Vec<Value>) -> Result<Value, RuntimeError> {
    let val = pop_stack(stack)?;

    Ok(Value::Float(match val {
        Value::Int(val) => (val as f64).atan(),
        Value::Float(val) => val.atan(),
        _ => panic!("dango stdlib `math-atan` error: value {val} is invalid"),
    }))
}

fn dango_math_cos(stack: &mut Vec<Value>) -> Result<Value, RuntimeError> {
    let val = pop_stack(stack)?;

    Ok(Value::Float(match val {
        Value::Int(val) => (val as f64).cos(),
        Value::Float(val) => val.cos(),
        _ => panic!("dango stdlib `math-cos` error: value {val} is invalid"),
    }))
}

fn dango_math_logb(stack: &mut Vec<Value>) -> Result<Value, RuntimeError> {
    let base = pop_stack(stack)?;
    let x = pop_stack(stack)?;

    if let Value::Float(base) = base {
        return Ok(Value::Float(
            match x {
                Value::Int(val) => val as f64,
                Value::Float(val) => val,
                _ => unimplemented!(),
            }.ln() / base.ln()
        ));
    }

    if let Value::Float(x) = x {
        return Ok(Value::Float(
            x.ln() /
            match base {
                Value::Int(val) => val as f64,
                Value::Float(val) => val,
                _ => unimplemented!(),
            }.ln()
        ));
    }

    if let (Value::Int(x), Value::Int(base)) = (x, base) {
        return Ok(Value::Float((x as f64).ln() / (base as f64).ln()));
    }

    unreachable!("dango stdlib `math-logb` internal error: unreachable case is reachable");
}

fn dango_math_pi(_: &mut Vec<Value>) -> Result<Value, RuntimeError> {
    Ok(Value::Float(3.14159265358979323846264338327950288419))
}

fn dango_math_pow(stack: &mut Vec<Value>) -> Result<Value, RuntimeError> {
    let exp = pop_stack(stack)?; 
    let base = pop_stack(stack)?;

    if let Value::Float(exp) = exp {
        return Ok(Value::Float(
            match base {
                Value::Int(val) => val as f64,
                Value::Float(val) => val,
                _ => unimplemented!(),
            }.powf(exp)
        ));
    }

    if let Value::Float(base) = base {
        return Ok(Value::Float(
            base.powf(
                match exp {
                    Value::Int(val) => val as f64,
                    Value::Float(val) => val,
                    _ => unimplemented!(),
                }
            )
        ));
    }

    if let (Value::Int(exp), Value::Int(base)) = (exp, base) {
        return Ok(Value::Int(base.pow(exp.try_into().unwrap())));
    }

    unreachable!("dango stdlib `math-pow` internal error: unreachable case is reachable");
}

fn dango_math_sin(stack: &mut Vec<Value>) -> Result<Value, RuntimeError> {
    let val = pop_stack(stack)?;

    Ok(Value::Float(match val {
        Value::Int(val) => (val as f64).sin(),
        Value::Float(val) => val.sin(),
        _ => panic!("dango stdlib `math-sin` error: value {val} is invalid"),
    }))
}

fn dango_math_sqrt(stack: &mut Vec<Value>) -> Result<Value, RuntimeError> {
    let val = pop_stack(stack)?;
    Ok(Value::Float(match val {
        Value::Int(val) => (val as f64).sqrt(),
        Value::Float(val) => val.sqrt(),
        _ => panic!("dango stdlib `math-sqrt` error: value {val} is invalid"),
    }))
}

fn dango_math_sqrt2(_: &mut Vec<Value>) -> Result<Value, RuntimeError> {
    Ok(Value::Float(1.4142135623730950488016887242096980785696))
}

fn dango_math_tan(stack: &mut Vec<Value>) -> Result<Value, RuntimeError> {
    let val = pop_stack(stack)?;

    Ok(Value::Float(match val {
        Value::Int(val) => (val as f64).tan(),
        Value::Float(val) => val.tan(),
        _ => panic!("dango stdlib `math-tan` error: value {val} is invalid"),
    }))
}

fn dango_env_args(_: &mut Vec<Value>) -> Result<Value, RuntimeError> {
    let args = std::env::args();
    let args = args.collect::<Vec<String>>();

    let mut dango = vec![];

    for arg in args {
        dango.push(Value::String(arg));
    }

    Ok(Value::Dango(dango))
}
