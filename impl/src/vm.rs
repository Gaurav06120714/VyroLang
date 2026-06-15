//! The VyroVM: a stack-based bytecode interpreter with call frames.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::opcode::Op;
use crate::value::{Class, Function, Instance, Value};

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
                Op::NewArray(n) => {
                    let start = self.stack.len() - n;
                    let elems: Vec<Value> = self.stack.drain(start..).collect();
                    self.stack.push(Value::Array(Rc::new(RefCell::new(elems))));
                }
                Op::IndexGet => {
                    let idx = self.pop();
                    let obj = self.pop();
                    let v = self.index_get(&obj, &idx)?;
                    self.stack.push(v);
                }
                Op::IndexSet => {
                    let val = self.pop();
                    let idx = self.pop();
                    let obj = self.pop();
                    self.index_set(&obj, &idx, val.clone())?;
                    self.stack.push(val);
                }
                Op::Class(i) => {
                    let name = self.name_const(i);
                    let class = Class { name, methods: HashMap::new() };
                    self.stack.push(Value::Class(Rc::new(class)));
                }
                Op::Method(i) => {
                    let name = self.name_const(i);
                    let func = match self.pop() {
                        Value::Func(f) => f,
                        _ => return Err(self.rt_err("method value is not a function")),
                    };
                    match self.stack.last() {
                        Some(Value::Class(c)) => {
                            // The class is still being defined; rebuild it with the new method.
                            let mut methods = c.methods.clone();
                            methods.insert(name, func);
                            let new_class =
                                Rc::new(Class { name: c.name.clone(), methods });
                            *self.stack.last_mut().unwrap() = Value::Class(new_class);
                        }
                        _ => return Err(self.rt_err("Method opcode without a class on the stack")),
                    }
                }
                Op::GetProp(i) => {
                    let name = self.name_const(i);
                    let obj = self.pop();
                    match obj {
                        Value::Instance(inst) => {
                            let v = inst.borrow().fields.get(&name).cloned();
                            match v {
                                Some(val) => self.stack.push(val),
                                None => {
                                    return Err(self.rt_err(format!(
                                        "undefined property '{}'",
                                        name
                                    )))
                                }
                            }
                        }
                        other => {
                            return Err(self.rt_err(format!(
                                "cannot read property '{}' of {}",
                                name,
                                other.type_name()
                            )))
                        }
                    }
                }
                Op::SetProp(i) => {
                    let name = self.name_const(i);
                    let val = self.pop();
                    let obj = self.pop();
                    match obj {
                        Value::Instance(inst) => {
                            inst.borrow_mut().fields.insert(name, val.clone());
                            self.stack.push(val);
                        }
                        other => {
                            return Err(self.rt_err(format!(
                                "cannot set property '{}' of {}",
                                name,
                                other.type_name()
                            )))
                        }
                    }
                }
                Op::Invoke(name_i, argc) => {
                    let name = self.name_const(name_i);
                    let recv_idx = self.stack.len() - argc - 1;
                    let recv = self.stack[recv_idx].clone();
                    match recv {
                        Value::Instance(inst) => {
                            let class = inst.borrow().class.clone();
                            match class.methods.get(&name) {
                                Some(m) => {
                                    if m.arity != argc {
                                        return Err(self.rt_err(format!(
                                            "method '{}' expects {} argument(s), got {}",
                                            name, m.arity, argc
                                        )));
                                    }
                                    let base = recv_idx; // slot 0 = receiver (self)
                                    self.frames.push(Frame { func: m.clone(), ip: 0, base });
                                }
                                None => {
                                    return Err(self.rt_err(format!(
                                        "undefined method '{}' on {}",
                                        name, class.name
                                    )))
                                }
                            }
                        }
                        other => {
                            return Err(self.rt_err(format!(
                                "cannot call method '{}' on {}",
                                name,
                                other.type_name()
                            )))
                        }
                    }
                }
                Op::Native(id, argc) => {
                    let start = self.stack.len() - argc;
                    let args: Vec<Value> = self.stack.drain(start..).collect();
                    let result = self.call_native(id, args)?;
                    self.stack.push(result);
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
                            self.frames.push(Frame { func: f, ip: 0, base: callee_idx });
                        }
                        Value::Class(c) => {
                            let instance = Rc::new(RefCell::new(Instance {
                                class: c.clone(),
                                fields: HashMap::new(),
                            }));
                            match c.methods.get("init") {
                                Some(init) => {
                                    if init.arity != argc {
                                        return Err(self.rt_err(format!(
                                            "{}.init expects {} argument(s), got {}",
                                            c.name, init.arity, argc
                                        )));
                                    }
                                    // put the instance in the slot-0 (self) position
                                    self.stack[callee_idx] = Value::Instance(instance);
                                    self.frames.push(Frame {
                                        func: init.clone(),
                                        ip: 0,
                                        base: callee_idx,
                                    });
                                }
                                None => {
                                    if argc != 0 {
                                        return Err(self.rt_err(format!(
                                            "class {} has no init but was called with {} argument(s)",
                                            c.name, argc
                                        )));
                                    }
                                    self.stack.truncate(callee_idx);
                                    self.stack.push(Value::Instance(instance));
                                }
                            }
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
                    // drop locals, args, and the callee/self slot
                    self.stack.truncate(frame.base);
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

    // ---- Collections ----

    fn as_index(&self, v: &Value) -> Result<usize, String> {
        match v {
            Value::Int(n) if *n >= 0 => Ok(*n as usize),
            Value::Int(n) => Err(self.rt_err(format!("negative index {}", n))),
            other => Err(self.rt_err(format!("index must be Int, got {}", other.type_name()))),
        }
    }

    fn index_get(&self, obj: &Value, idx: &Value) -> Result<Value, String> {
        let i = self.as_index(idx)?;
        match obj {
            Value::Array(a) => {
                let b = a.borrow();
                b.get(i)
                    .cloned()
                    .ok_or_else(|| self.rt_err(format!("array index {} out of bounds (len {})", i, b.len())))
            }
            Value::Str(s) => {
                let ch = s.chars().nth(i);
                ch.map(|c| Value::Str(Rc::new(c.to_string())))
                    .ok_or_else(|| self.rt_err(format!("string index {} out of bounds", i)))
            }
            other => Err(self.rt_err(format!("{} is not indexable", other.type_name()))),
        }
    }

    fn index_set(&self, obj: &Value, idx: &Value, val: Value) -> Result<(), String> {
        let i = self.as_index(idx)?;
        match obj {
            Value::Array(a) => {
                let mut b = a.borrow_mut();
                if i >= b.len() {
                    return Err(self.rt_err(format!(
                        "array index {} out of bounds (len {})",
                        i,
                        b.len()
                    )));
                }
                b[i] = val;
                Ok(())
            }
            other => Err(self.rt_err(format!("cannot index-assign {}", other.type_name()))),
        }
    }

    // ---- Native (built-in) functions; ids must match `compiler::native_id` ----

    fn call_native(&self, id: usize, args: Vec<Value>) -> Result<Value, String> {
        let nargs = |want: usize, name: &str| -> Result<(), String> {
            if args.len() != want {
                Err(self.rt_err(format!("{}() expects {} argument(s), got {}", name, want, args.len())))
            } else {
                Ok(())
            }
        };
        let as_f64 = |v: &Value| -> Result<f64, String> {
            match v {
                Value::Int(n) => Ok(*n as f64),
                Value::Float(f) => Ok(*f),
                other => Err(self.rt_err(format!("expected a number, got {}", other.type_name()))),
            }
        };
        match id {
            0 => {
                // len
                nargs(1, "len")?;
                match &args[0] {
                    Value::Array(a) => Ok(Value::Int(a.borrow().len() as i64)),
                    Value::Str(s) => Ok(Value::Int(s.chars().count() as i64)),
                    other => Err(self.rt_err(format!("len() needs Array or String, got {}", other.type_name()))),
                }
            }
            1 => {
                // push(array, value)
                nargs(2, "push")?;
                match &args[0] {
                    Value::Array(a) => {
                        a.borrow_mut().push(args[1].clone());
                        Ok(Value::Null)
                    }
                    other => Err(self.rt_err(format!("push() needs an Array, got {}", other.type_name()))),
                }
            }
            2 => {
                // pop(array)
                nargs(1, "pop")?;
                match &args[0] {
                    Value::Array(a) => Ok(a.borrow_mut().pop().unwrap_or(Value::Null)),
                    other => Err(self.rt_err(format!("pop() needs an Array, got {}", other.type_name()))),
                }
            }
            3 => {
                nargs(1, "str")?;
                Ok(Value::Str(Rc::new(args[0].to_string())))
            }
            4 => {
                // int
                nargs(1, "int")?;
                match &args[0] {
                    Value::Int(n) => Ok(Value::Int(*n)),
                    Value::Float(f) => Ok(Value::Int(*f as i64)),
                    Value::Bool(b) => Ok(Value::Int(if *b { 1 } else { 0 })),
                    Value::Str(s) => s
                        .trim()
                        .parse::<i64>()
                        .map(Value::Int)
                        .map_err(|_| self.rt_err(format!("int() cannot parse '{}'", s))),
                    other => Err(self.rt_err(format!("int() cannot convert {}", other.type_name()))),
                }
            }
            5 => {
                // float
                nargs(1, "float")?;
                match &args[0] {
                    Value::Int(n) => Ok(Value::Float(*n as f64)),
                    Value::Float(f) => Ok(Value::Float(*f)),
                    Value::Str(s) => s
                        .trim()
                        .parse::<f64>()
                        .map(Value::Float)
                        .map_err(|_| self.rt_err(format!("float() cannot parse '{}'", s))),
                    other => Err(self.rt_err(format!("float() cannot convert {}", other.type_name()))),
                }
            }
            6 => {
                // abs
                nargs(1, "abs")?;
                match &args[0] {
                    Value::Int(n) => Ok(Value::Int(n.abs())),
                    Value::Float(f) => Ok(Value::Float(f.abs())),
                    other => Err(self.rt_err(format!("abs() needs a number, got {}", other.type_name()))),
                }
            }
            7 => {
                nargs(1, "sqrt")?;
                Ok(Value::Float(as_f64(&args[0])?.sqrt()))
            }
            8 => {
                // floor
                nargs(1, "floor")?;
                match &args[0] {
                    Value::Int(n) => Ok(Value::Int(*n)),
                    Value::Float(f) => Ok(Value::Int(f.floor() as i64)),
                    other => Err(self.rt_err(format!("floor() needs a number, got {}", other.type_name()))),
                }
            }
            9 => {
                // ceil
                nargs(1, "ceil")?;
                match &args[0] {
                    Value::Int(n) => Ok(Value::Int(*n)),
                    Value::Float(f) => Ok(Value::Int(f.ceil() as i64)),
                    other => Err(self.rt_err(format!("ceil() needs a number, got {}", other.type_name()))),
                }
            }
            10 => {
                // pow(base, exp)
                nargs(2, "pow")?;
                match (&args[0], &args[1]) {
                    (Value::Int(b), Value::Int(e)) if *e >= 0 => {
                        Ok(Value::Int(b.pow(*e as u32)))
                    }
                    _ => Ok(Value::Float(as_f64(&args[0])?.powf(as_f64(&args[1])?))),
                }
            }
            11 | 12 => {
                // min / max
                let name = if id == 11 { "min" } else { "max" };
                if args.is_empty() {
                    return Err(self.rt_err(format!("{}() needs at least one argument", name)));
                }
                let mut best = as_f64(&args[0])?;
                let mut best_v = args[0].clone();
                for a in &args[1..] {
                    let x = as_f64(a)?;
                    let take = if id == 11 { x < best } else { x > best };
                    if take {
                        best = x;
                        best_v = a.clone();
                    }
                }
                Ok(best_v)
            }
            13 => {
                nargs(1, "upper")?;
                match &args[0] {
                    Value::Str(s) => Ok(Value::Str(Rc::new(s.to_uppercase()))),
                    other => Err(self.rt_err(format!("upper() needs a String, got {}", other.type_name()))),
                }
            }
            14 => {
                nargs(1, "lower")?;
                match &args[0] {
                    Value::Str(s) => Ok(Value::Str(Rc::new(s.to_lowercase()))),
                    other => Err(self.rt_err(format!("lower() needs a String, got {}", other.type_name()))),
                }
            }
            15 => {
                nargs(1, "type")?;
                Ok(Value::Str(Rc::new(args[0].type_name().to_string())))
            }
            16 => {
                // input(): read one line from stdin; null at EOF
                nargs(0, "input")?;
                use std::io::BufRead;
                let mut line = String::new();
                let n = std::io::stdin()
                    .lock()
                    .read_line(&mut line)
                    .map_err(|e| self.rt_err(format!("input() failed: {}", e)))?;
                if n == 0 {
                    Ok(Value::Null)
                } else {
                    let trimmed = line.trim_end_matches(['\n', '\r']);
                    Ok(Value::Str(Rc::new(trimmed.to_string())))
                }
            }
            _ => Err(self.rt_err(format!("unknown native function #{}", id))),
        }
    }
}
