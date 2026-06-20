//! Runtime values for the VyroVM.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::opcode::Chunk;

#[derive(Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(Rc<String>),
    Array(Rc<RefCell<Vec<Value>>>),
    Map(Rc<RefCell<std::collections::BTreeMap<String, Value>>>),
    Func(Rc<Function>),
    Class(Rc<Class>),
    Instance(Rc<RefCell<Instance>>),
    Null,
}

pub struct Function {
    pub name: String,
    pub arity: usize,
    pub chunk: Chunk,
}

pub struct Class {
    pub name: String,
    pub methods: HashMap<String, Rc<Function>>,
}

pub struct Instance {
    pub class: Rc<Class>,
    pub fields: HashMap<String, Value>,
}

impl Value {
    pub fn truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Null => false,
            Value::Int(n) => *n != 0,
            Value::Float(f) => *f != 0.0,
            Value::Str(s) => !s.is_empty(),
            Value::Array(a) => !a.borrow().is_empty(),
            Value::Map(m) => !m.borrow().is_empty(),
            Value::Func(_) | Value::Class(_) | Value::Instance(_) => true,
        }
    }

    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "Int",
            Value::Float(_) => "Float",
            Value::Bool(_) => "Bool",
            Value::Str(_) => "String",
            Value::Array(_) => "Array",
            Value::Map(_) => "Map",
            Value::Func(_) => "Function",
            Value::Class(_) => "Class",
            Value::Instance(_) => "instance",
            Value::Null => "null",
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Float(x) => {
                if x.fract() == 0.0 && x.is_finite() {
                    write!(f, "{:.1}", x)
                } else {
                    write!(f, "{}", x)
                }
            }
            Value::Bool(b) => write!(f, "{}", b),
            Value::Str(s) => write!(f, "{}", s),
            Value::Array(a) => {
                let items = a.borrow();
                let parts: Vec<String> = items.iter().map(render_elem).collect();
                write!(f, "[{}]", parts.join(", "))
            }
            Value::Map(m) => {
                let items = m.borrow();
                let parts: Vec<String> =
                    items.iter().map(|(k, v)| format!("\"{}\": {}", k, render_elem(v))).collect();
                write!(f, "{{{}}}", parts.join(", "))
            }
            Value::Func(func) => write!(f, "<func {}>", func.name),
            Value::Class(c) => write!(f, "<class {}>", c.name),
            Value::Instance(i) => write!(f, "<{} instance>", i.borrow().class.name),
            Value::Null => write!(f, "null"),
        }
    }
}

/// Render an element inside an array literal (strings get quotes for clarity).
fn render_elem(v: &Value) -> String {
    match v {
        Value::Str(s) => format!("\"{}\"", s),
        other => other.to_string(),
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Int(a), Value::Int(b)) => a == b,
            (Value::Float(a), Value::Float(b)) => a == b,
            (Value::Int(a), Value::Float(b)) => (*a as f64) == *b,
            (Value::Float(a), Value::Int(b)) => *a == (*b as f64),
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Str(a), Value::Str(b)) => a == b,
            (Value::Array(a), Value::Array(b)) => *a.borrow() == *b.borrow(),
            (Value::Map(a), Value::Map(b)) => *a.borrow() == *b.borrow(),
            (Value::Instance(a), Value::Instance(b)) => Rc::ptr_eq(a, b),
            (Value::Class(a), Value::Class(b)) => Rc::ptr_eq(a, b),
            (Value::Func(a), Value::Func(b)) => Rc::ptr_eq(a, b),
            (Value::Null, Value::Null) => true,
            _ => false,
        }
    }
}
