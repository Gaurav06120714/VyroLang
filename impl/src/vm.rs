//! The VyroVM: a stack-based bytecode interpreter with call frames.

use std::collections::HashMap;
use std::rc::Rc;

use crate::opcode::Op;
use crate::value::{Function, Value};

struct Frame {
    func: Rc<Function>,
    ip: usize,
    base: usize, // stack index of this frame's local slot 0
}

pub struct Vm {
    stack: Vec<Value>,
    frames: Vec<Frame>,
    globals: HashMap<String, Value>,
    out: String,
}

impl Vm {
    pub fn new() -> Self {
        Vm { stack: Vec::new(), frames: Vec::new(), globals: HashMap::new(), out: String::new() }
    }

    /// Runs `main`, returning everything printed (also echoed to stdout live).
    pub fn run(&mut self, main: Function) -> Result<String, String> {
        let main = Rc::new(main);
        self.frames.push(Frame { func: main, ip: 0, base: 0 });
        self.exec()?;
        Ok(std::mem::take(&mut self.out))
    }

    fn rt_err(&self, msg: impl Into<String>) -> String {
        format!("runtime error: {}", msg.into())
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().expect("stack underflow")
    }

    fn peek(&self) -> &Value {
        self.stack.last().expect("stack underflow")
    }

    fn const_at(&self, i: usize) -> Value {
        self.frames.last().unwrap().func.chunk.constants[i].clone()
    }

    fn name_const(&self, i: usize) -> String {
        match self.const_at(i) {
            Value::Str(s) => (*s).clone(),
            _ => unreachable!("name constant is not a string"),
        }
    }

    fn exec(&mut self) -> Result<(), String> {
        loop {
            let fi = self.frames.len() - 1;
            let ip = self.frames[fi].ip;
            let op = self.frames[fi].func.chunk.code[ip].clone();
            self.frames[fi].ip += 1;

            match op {
                Op::Const(i) => {
                    let v = self.const_at(i);
                    self.stack.push(v);
                }
                Op::True => self.stack.push(Value::Bool(true)),
                Op::False => self.stack.push(Value::Bool(false)),
                Op::Null => self.stack.push(Value::Null),
                Op::Pop => {
                    self.pop();
                }
                Op::DefineGlobal(i) => {
                    let name = self.name_const(i);
                    let v = self.pop();
                    self.globals.insert(name, v);
                }
                Op::GetGlobal(i) => {
                    let name = self.name_const(i);
                    match self.globals.get(&name) {
                        Some(v) => self.stack.push(v.clone()),
                        None => return Err(self.rt_err(format!("undefined variable '{}'", name))),
                    }
                }
                Op::SetGlobal(i) => {
                    let name = self.name_const(i);
                    if !self.globals.contains_key(&name) {
                        return Err(self.rt_err(format!("assignment to undefined variable '{}'", name)));
                    }
                    let v = self.peek().clone();
                    self.globals.insert(name, v);
                }
                Op::GetLocal(slot) => {
                    let base = self.frames[fi].base;
                    self.stack.push(self.stack[base + slot].clone());
                }
                Op::SetLocal(slot) => {
                    let base = self.frames[fi].base;
                    let v = self.peek().clone();
                    self.stack[base + slot] = v;
                }
                Op::Add => self.binary_add()?,
                Op::Sub => self.binary_num(|a, b| a - b, |a, b| a - b, "-")?,
                Op::Mul => self.binary_num(|a, b| a * b, |a, b| a * b, "*")?,
                Op::Div => self.binary_div()?,
                Op::Mod => self.binary_mod()?,
                Op::Neg => {
                    let v = self.pop();
                    match v {
                        Value::Int(n) => self.stack.push(Value::Int(-n)),
                        Value::Float(f) => self.stack.push(Value::Float(-f)),
                        other => return Err(self.rt_err(format!("cannot negate {}", other.type_name()))),
                    }
                }
                Op::Eq => {
                    let b = self.pop();
                    let a = self.pop();
                    self.stack.push(Value::Bool(a == b));
                }
                Op::Ne => {
                    let b = self.pop();
                    let a = self.pop();
                    self.stack.push(Value::Bool(a != b));
                }
                Op::Lt => self.binary_cmp(|o| o.is_lt())?,
                Op::Le => self.binary_cmp(|o| o.is_le())?,
                Op::Gt => self.binary_cmp(|o| o.is_gt())?,
                Op::Ge => self.binary_cmp(|o| o.is_ge())?,
                Op::Not => {
                    let v = self.pop();
                    self.stack.push(Value::Bool(!v.truthy()));
                }
                Op::Jump(t) => self.frames[fi].ip = t,
                Op::JumpIfFalse(t) => {
                    let v = self.pop();
                    if !v.truthy() {
                        self.frames[fi].ip = t;
                    }
                }
                Op::JumpIfFalsePeek(t) => {
                    if !self.peek().truthy() {
                        self.frames[fi].ip = t;
                    }
                }
                Op::JumpIfTruePeek(t) => {
                    if self.peek().truthy() {
                        self.frames[fi].ip = t;
                    }
                }
                Op::Print(argc) => {
                    let start = self.stack.len() - argc;
                    let parts: Vec<String> =
                        self.stack.drain(start..).map(|v| v.to_string()).collect();
                    let line = parts.join(" ");
                    println!("{}", line);
                    self.out.push_str(&line);
                    self.out.push('\n');
                    self.stack.push(Value::Null);
                }
                Op::Call(argc) => {
                    let callee_idx = self.stack.len() - argc - 1;
                    let callee = self.stack[callee_idx].clone();
                    match callee {
                        Value::Func(f) => {
                            if f.arity != argc {
                                return Err(self.rt_err(format!(
                                    "function '{}' expects {} argument(s), got {}",
                                    f.name, f.arity, argc
                                )));
                            }
                            let base = self.stack.len() - argc;
                            self.frames.push(Frame { func: f, ip: 0, base });
                        }
                        other => {
                            return Err(self.rt_err(format!("'{}' is not callable", other.type_name())))
                        }
                    }
                }
                Op::Return => {
                    let ret = self.pop();
                    let frame = self.frames.pop().unwrap();
                    if self.frames.is_empty() {
                        // returning from main
                        self.stack.clear();
                        return Ok(());
                    }
                    // drop locals, args, and the callee slot
                    self.stack.truncate(frame.base - 1);
                    self.stack.push(ret);
                }
            }
        }
    }

    // ---- Arithmetic helpers ----

    fn binary_add(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        let v = match (&a, &b) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
            (Value::Float(x), Value::Float(y)) => Value::Float(x + y),
            (Value::Int(x), Value::Float(y)) => Value::Float(*x as f64 + y),
            (Value::Float(x), Value::Int(y)) => Value::Float(x + *y as f64),
            // string concatenation if either side is a string
            (Value::Str(_), _) | (_, Value::Str(_)) => {
                Value::Str(Rc::new(format!("{}{}", a, b)))
            }
            _ => {
                return Err(self.rt_err(format!(
                    "cannot add {} and {}",
                    a.type_name(),
                    b.type_name()
                )))
            }
        };
        self.stack.push(v);
        Ok(())
    }

    fn binary_num(
        &mut self,
        fi: fn(i64, i64) -> i64,
        ff: fn(f64, f64) -> f64,
        sym: &str,
    ) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        let v = match (&a, &b) {
            (Value::Int(x), Value::Int(y)) => Value::Int(fi(*x, *y)),
            (Value::Float(x), Value::Float(y)) => Value::Float(ff(*x, *y)),
            (Value::Int(x), Value::Float(y)) => Value::Float(ff(*x as f64, *y)),
            (Value::Float(x), Value::Int(y)) => Value::Float(ff(*x, *y as f64)),
            _ => {
                return Err(self.rt_err(format!(
                    "cannot apply '{}' to {} and {}",
                    sym,
                    a.type_name(),
                    b.type_name()
                )))
            }
        };
        self.stack.push(v);
        Ok(())
    }

    fn binary_div(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        let v = match (&a, &b) {
            (Value::Int(_), Value::Int(0)) => return Err(self.rt_err("division by zero")),
            (Value::Int(x), Value::Int(y)) => Value::Int(x / y),
            (Value::Float(x), Value::Float(y)) => Value::Float(x / y),
            (Value::Int(x), Value::Float(y)) => Value::Float(*x as f64 / y),
            (Value::Float(x), Value::Int(y)) => Value::Float(x / *y as f64),
            _ => {
                return Err(self.rt_err(format!(
                    "cannot divide {} by {}",
                    a.type_name(),
                    b.type_name()
                )))
            }
        };
        self.stack.push(v);
        Ok(())
    }

    fn binary_mod(&mut self) -> Result<(), String> {
        let b = self.pop();
        let a = self.pop();
        let v = match (&a, &b) {
            (Value::Int(_), Value::Int(0)) => return Err(self.rt_err("modulo by zero")),
            (Value::Int(x), Value::Int(y)) => Value::Int(x % y),
            (Value::Float(x), Value::Float(y)) => Value::Float(x % y),
            (Value::Int(x), Value::Float(y)) => Value::Float(*x as f64 % y),
            (Value::Float(x), Value::Int(y)) => Value::Float(x % *y as f64),
            _ => {
                return Err(self.rt_err(format!(
                    "cannot apply '%' to {} and {}",
                    a.type_name(),
                    b.type_name()
                )))
            }
        };
        self.stack.push(v);
        Ok(())
    }

    fn binary_cmp(&mut self, pick: fn(std::cmp::Ordering) -> bool) -> Result<(), String> {
        use std::cmp::Ordering;
        let b = self.pop();
        let a = self.pop();
        let ord = match (&a, &b) {
            (Value::Int(x), Value::Int(y)) => x.cmp(y),
            (Value::Float(x), Value::Float(y)) => x.partial_cmp(y).unwrap_or(Ordering::Equal),
            (Value::Int(x), Value::Float(y)) => {
                (*x as f64).partial_cmp(y).unwrap_or(Ordering::Equal)
            }
            (Value::Float(x), Value::Int(y)) => {
                x.partial_cmp(&(*y as f64)).unwrap_or(Ordering::Equal)
            }
            (Value::Str(x), Value::Str(y)) => x.cmp(y),
            _ => {
                return Err(self.rt_err(format!(
                    "cannot compare {} and {}",
                    a.type_name(),
                    b.type_name()
                )))
            }
        };
        self.stack.push(Value::Bool(pick(ord)));
        Ok(())
    }
}
