use crate::error::Result;
use crate::model::{Location, Value};
use std::rc::Rc;

pub fn println(arg: Value, _: Rc<Location>) -> Result<Value> {
    println!("{arg}");
    Ok(arg)
}
