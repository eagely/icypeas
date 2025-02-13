use super::enums::Value;
use crate::error::{Error, ErrorKind, Result};
use crate::lexer::enums::Location;
use std::collections::HashMap;
use std::rc::Rc;

#[macro_export]
macro_rules! builtin {
    ($name:ident, $location:ident, $($arg_name:ident),* => $body:block) => {
        pub fn $name(args: Vec<Value>, $location: Rc<Location>) -> Result<Value> {
            let mut iter = args.into_iter();
            $(
                let $arg_name = iter.next().ok_or_else(|| Error::with_help(ErrorKind::InvalidArguments, Rc::clone(&$location), "Expected more arguments"))?;
            )*
            $body
        }
    };
}

#[macro_export]
macro_rules! add_builtins {
    ($($name:ident),*) => {
        pub fn add_builtins() -> HashMap<String, fn(Vec<Value>, Rc<Location>) -> Result<Value>> {
            let mut builtins = HashMap::new();
            $(
                builtins.insert(stringify!($name).to_string(), $name as fn(Vec<Value>, Rc<Location>) -> Result<Value>);
            )*
            builtins
        }
    };
}

builtin!(print, location, value => {
    println!("{}", value);
    Ok(Value::None)
});

builtin!(add, location, a, b => {
    match (&a, &b) {
        (Value::Integer(a), Value::Integer(b)) => a.checked_add(*b)
            .map(Value::Integer)
            .ok_or_else(|| Error::new(ErrorKind::Overflow, Rc::clone(&location))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
        (Value::String(a), Value::String(b)) => Ok(Value::String(a.clone() + b)),
        (Value::String(a), Value::Integer(b)) => Ok(Value::String(a.clone() + &b.to_string())),
        (Value::String(a), Value::Float(b)) => Ok(Value::String(a.clone() + &b.to_string())),
        (Value::String(a), Value::Boolean(b)) => Ok(Value::String(a.clone() + &b.to_string())),
        (Value::Integer(a), Value::String(b)) => Ok(Value::String(a.to_string() + b)),
        (Value::Float(a), Value::String(b)) => Ok(Value::String(a.to_string() + b)),
        (Value::Boolean(a), Value::String(b)) => Ok(Value::String(a.to_string() + b)),
        _ => Err(Error::with_help(ErrorKind::MismatchedTypes, Rc::clone(&location), &format!("Unsupported types for addition: {} and {}", a, b))),
    }
});

builtin!(sub, location, a, b => {
    match (&a, &b) {
        (Value::Integer(a), Value::Integer(b)) => a.checked_sub(*b)
            .map(Value::Integer)
            .ok_or_else(|| Error::new(ErrorKind::Overflow, Rc::clone(&location))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
        _ => Err(Error::with_help(ErrorKind::MismatchedTypes, Rc::clone(&location), &format!("Unsupported types for subtraction: {} and {}", a, b))),
    }
});

builtin!(mul, location, a, b => {
    match (&a, &b) {
        (Value::Integer(a), Value::Integer(b)) => a.checked_mul(*b)
            .map(Value::Integer)
            .ok_or_else(|| Error::new(ErrorKind::Overflow, Rc::clone(&location))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
        _ => Err(Error::with_help(ErrorKind::MismatchedTypes, Rc::clone(&location), &format!("Unsupported types for multiplication: {} and {}", a, b))),
    }
});

builtin!(div, location, a, b => {
    match (&a, &b) {
        (Value::Integer(a), Value::Integer(b)) => {
            if *b == 0 {
                Err(Error::new(ErrorKind::DivisionByZero, Rc::clone(&location)))
            } else {
                a.checked_div(*b)
                    .map(Value::Integer)
                    .ok_or_else(|| Error::new(ErrorKind::Overflow, Rc::clone(&location)))
            }
        },
        (Value::Float(a), Value::Float(b)) => {
            if *b == 0.0 {
                Err(Error::new(ErrorKind::DivisionByZero, Rc::clone(&location)))
            } else {
                Ok(Value::Float(a / b))
            }
        },
        _ => Err(Error::with_help(ErrorKind::MismatchedTypes, Rc::clone(&location), &format!("Unsupported types for division: {} and {}", a, b))),
    }
});

builtin!(modulo, location, a, b => {
    match (&a, &b) {
        (Value::Integer(a), Value::Integer(b)) => a.checked_rem(*b)
            .map(Value::Integer)
            .ok_or_else(|| Error::new(ErrorKind::Overflow, Rc::clone(&location))),
        (Value::Float(a), Value::Float(b)) => Ok(Value::Float(a % b)),
        _ => Err(Error::with_help(ErrorKind::MismatchedTypes, Rc::clone(&location), &format!("Unsupported types for modulo: {} and {}", a, b))),
    }
});

add_builtins!(print, add, sub, mul, div, modulo);
