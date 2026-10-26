use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value {
    Float(f64),
    Bool(bool),
    Int(usize),
    Signed(isize),
    String(String),
    Struct(HashMap<String, Value>), // Struct as key-value pairs
    Vec(Vec<Value>),
}

impl Value {
    pub fn float(val: f64) -> Self {
        Value::Float(val)
    }

    pub fn int(val: usize) -> Self {
        Value::Int(val)
    }

    pub fn signed(val: isize) -> Self {
        Value::Signed(val)
    }

    pub fn string(val: impl Into<String>) -> Self {
        Value::String(val.into())
    }

    pub fn struct_fields(fields: HashMap<String, Value>) -> Self {
        Value::Struct(fields)
    }

    pub fn vec(vals: Vec<Value>) -> Self {
        Value::Vec(vals)
    }
}
