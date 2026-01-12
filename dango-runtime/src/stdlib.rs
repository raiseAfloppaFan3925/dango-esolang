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
    runtime.register_function("math-e".to_string(), dango_math_e);
    runtime.register_function("math-logb".to_string(), dango_math_logb);
    runtime.register_function("math-pi".to_string(), dango_math_pi);
    runtime.register_function("math-pow".to_string(), dango_math_pow);
    runtime.register_function("math-sin".to_string(), dango_math_sin);
    runtime.register_function("math-sqrt".to_string(), dango_math_sqrt);
    runtime.register_function("math-sqrt2".to_string(), dango_math_sqrt2);
    runtime.register_function("math-tan".to_string(), dango_math_tan);
}

pub fn load_chrono(runtime: &mut Runtime) {
    runtime.register_function("chrono-now".to_string(), dango_chrono_now);
    runtime.register_function("chrono-sleep".to_string(), dango_chrono_sleep);
}

fn dango_chrono_now(_: &mut Runtime) -> Result<Value, RuntimeError> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now();
    let now = now.duration_since(UNIX_EPOCH)
        .ok().ok_or(RuntimeError::CustomError("`chrono-now` internal error: could not get system time".to_string()))?;

    let now = now.as_micros() as f64;

    Ok(Value::Float(now / 1_000_000.0))
}

fn dango_chrono_sleep(runtime: &mut Runtime) -> Result<Value, RuntimeError> {
    use std::time::Duration;
    use std::thread;

    match runtime.pop()? {
        Value::Int(dur) => match TryInto::<u64>::try_into(dur) {
            Ok(dur) => {
                let dur = Duration::from_secs(dur);

                thread::sleep(dur);
            }
            Err(_) => return Err(RuntimeError::CustomError("`:chrono-sleep` error: sleep duration must be positive".to_string())),
        }
        Value::Float(dur_f64) => {
            let dur = Duration::try_from_secs_f64(dur_f64)
                .ok().ok_or(RuntimeError::CustomError(format!("`:chrono-sleep` error: duration {} is invalid", dur_f64)))?;

            thread::sleep(dur);
        }
        _ => return Err(RuntimeError::CustomError("`:chrono-sleep` error: duration must be int or float".to_string())),
    }

    Ok(Value::Nil)
}

fn dango_io_input(runtime: &mut Runtime) -> Result<Value, RuntimeError> {
    use std::io;
    use std::io::BufRead;

    let mode = runtime.peek(0).cloned();
    
    if let Ok(Value::Int(mode)) = mode {
        runtime.pop()?;
        if mode == 1 {
            let prompt = runtime.pop()?;
            print!("{}", prompt);
            std::io::stdout().lock().flush().ok().ok_or(RuntimeError::CustomError("`:io-input` internal error: failed to print/flush prompt".to_string()))?;
        }
    }

    let mut stdin = io::stdin().lock();

    let mut read_value = String::new();
    let Ok(_) = stdin.read_line(&mut read_value) else {
        return Err(RuntimeError::CustomError("`:io-input` internal error: failed to read from input stream".to_string()));
    };

    // \r\n is very annoying
    read_value.truncate(read_value.rfind("\r").unwrap_or(read_value.len()));
    read_value.truncate(read_value.rfind("\n").unwrap_or(read_value.len()));

    Ok(Value::String(read_value))
}

fn dango_io_write(runtime: &mut Runtime) -> Result<Value, RuntimeError> {
    let Value::String(target) = runtime.pop()? else {
        return Err(RuntimeError::CustomError("`:io-write` error: target stream value is a string".to_string()));
    };

    let value = runtime.pop()?;

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

fn dango_math_abs(runtime: &mut Runtime) -> Result<Value, RuntimeError> {
    let val = runtime.pop()?;

    Ok(match val {
        Value::Int(val) => Value::Int(val.abs()),
        Value::Float(val) => Value::Float(val.abs()),
        _ => return Err(RuntimeError::CustomError("`:math-sqrt` error: value is invalid".to_string())),
    })
}

fn dango_math_asin(runtime: &mut Runtime) -> Result<Value, RuntimeError> {
    let val = runtime.pop()?;

    Ok(Value::Float(match val {
        Value::Int(val) => (val as f64).asin(),
        Value::Float(val) => val.asin(),
        _ => return Err(RuntimeError::CustomError("`:math-asin` error: value is invalid".to_string())),
    }))
}

fn dango_math_acos(runtime: &mut Runtime) -> Result<Value, RuntimeError> {
    let val = runtime.pop()?;

    Ok(Value::Float(match val {
        Value::Int(val) => (val as f64).acos(),
        Value::Float(val) => val.acos(),
        _ => return Err(RuntimeError::CustomError(format!("dango stdlib `math-acos` error: value {val} is invalid"))),
    }))
}

fn dango_math_atan(runtime: &mut Runtime) -> Result<Value, RuntimeError> {
    let val = runtime.pop()?;

    Ok(Value::Float(match val {
        Value::Int(val) => (val as f64).atan(),
        Value::Float(val) => val.atan(),
        _ => return Err(RuntimeError::CustomError(format!("dango stdlib `math-atan` error: value {val} is invalid"))),
    }))
}

fn dango_math_cos(runtime: &mut Runtime) -> Result<Value, RuntimeError> {
    let val = runtime.pop()?;

    Ok(Value::Float(match val {
        Value::Int(val) => (val as f64).cos(),
        Value::Float(val) => val.cos(),
        _ => return Err(RuntimeError::CustomError(format!("dango stdlib `math-cos` error: value {val} is invalid"))),
    }))
}

fn dango_math_e(_: &mut Runtime) -> Result<Value, RuntimeError> {
    Ok(Value::Float(2.71828182845904523536028747135266249775))
}

fn dango_math_logb(runtime: &mut Runtime) -> Result<Value, RuntimeError> {
    let base = runtime.pop()?;
    let x = runtime.pop()?;

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

fn dango_math_pi(_: &mut Runtime) -> Result<Value, RuntimeError> {
    Ok(Value::Float(3.14159265358979323846264338327950288419))
}

fn dango_math_pow(runtime: &mut Runtime) -> Result<Value, RuntimeError> {
    let exp = runtime.pop()?; 
    let base = runtime.pop()?;

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

fn dango_math_sin(runtime: &mut Runtime) -> Result<Value, RuntimeError> {
    let val = runtime.pop()?;

    Ok(Value::Float(match val {
        Value::Int(val) => (val as f64).sin(),
        Value::Float(val) => val.sin(),
        _ => return Err(RuntimeError::CustomError(format!("dango stdlib `math-sin` error: value {val} is invalid"))),
    }))
}

fn dango_math_sqrt(runtime: &mut Runtime) -> Result<Value, RuntimeError> {
    let val = runtime.pop()?;
    Ok(Value::Float(match val {
        Value::Int(val) => (val as f64).sqrt(),
        Value::Float(val) => val.sqrt(),
        _ => return Err(RuntimeError::CustomError(format!("dango stdlib `math-sqrt` error: value {val} is invalid"))),
    }))
}

fn dango_math_sqrt2(_: &mut Runtime) -> Result<Value, RuntimeError> {
    Ok(Value::Float(1.4142135623730950488016887242096980785696))
}

fn dango_math_tan(runtime: &mut Runtime) -> Result<Value, RuntimeError> {
    let val = runtime.pop()?;

    Ok(Value::Float(match val {
        Value::Int(val) => (val as f64).tan(),
        Value::Float(val) => val.tan(),
        _ => return Err(RuntimeError::CustomError(format!("dango stdlib `math-tan` error: value {val} is invalid"))),
    }))
}

fn dango_env_args(_: &mut Runtime) -> Result<Value, RuntimeError> {
    let args = std::env::args()
        .collect::<Vec<String>>()
        .iter().map(|arg| Value::String(arg.to_owned()))
        .collect::<Vec<Value>>();

    Ok(Value::dango_from_vec(args))
}
