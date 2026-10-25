use std::collections::HashMap;

#[derive(Debug)]
pub enum Value<'a> {
    Float(f64),
    Int(usize),
    Signed(isize),
    String(&'a str),
    Struct(HashMap<&'a str, Value<'a>>), // Struct as key-value pairs
    Vec(Vec<Value<'a>>),
}

impl<'a> Value<'a> {
    pub fn float(val: f64) -> Self {
        Value::Float(val)
    }

    pub fn int(val: usize) -> Self {
        Value::Int(val)
    }

    pub fn signed(val: isize) -> Self {
        Value::Signed(val)
    }

    pub fn string(val: &'a str) -> Self {
        Value::String(val)
    }

    pub fn struct_fields(fields: HashMap<&'a str, Value<'a>>) -> Self {
        Value::Struct(fields)
    }

    pub fn vec(vals: Vec<Value<'a>>) -> Self {
        Value::Vec(vals)
    }
}
