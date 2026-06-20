//! Compiler: AST -> bytecode (a top-level `main` Function plus nested functions).

use std::rc::Rc;

use crate::ast::*;
use crate::opcode::{Chunk, Op};
use crate::value::{Function, Value};

struct Local {
    name: String,
    depth: i32,
}

struct FuncState {
    chunk: Chunk,
    locals: Vec<Local>,
    scope_depth: i32,
    is_init: bool,
}

impl FuncState {
    fn new() -> Self {
        FuncState { chunk: Chunk::new(), locals: Vec::new(), scope_depth: 0, is_init: false }
    }
}

/// Native (built-in) functions, keyed by source name. Ids must match `vm.rs`.
fn native_id(name: &str) -> Option<usize> {
    Some(match name {
        "len" => 0,
        "push" => 1,
        "pop" => 2,
        "str" => 3,
        "int" => 4,
        "float" => 5,
        "abs" => 6,
        "sqrt" => 7,
        "floor" => 8,
        "ceil" => 9,
        "pow" => 10,
        "min" => 11,
        "max" => 12,
        "upper" => 13,
        "lower" => 14,
        "type" => 15,
        "input" => 16,
        "keys" => 17,
        "has" => 18,
        _ => return None,
    })
}

pub struct Compiler {
    stack: Vec<FuncState>,
}

impl Compiler {
    pub fn compile(program: &Program) -> Result<Function, String> {
        let mut c = Compiler { stack: vec![FuncState::new()] };
        for stmt in &program.stmts {
            c.stmt(stmt)?;
        }
        c.emit(Op::Null);
        c.emit(Op::Return);
        let fs = c.stack.pop().unwrap();
        Ok(Function { name: "main".to_string(), arity: 0, chunk: fs.chunk })
    }

    fn cur(&mut self) -> &mut FuncState {
        self.stack.last_mut().unwrap()
    }

    fn chunk(&mut self) -> &mut Chunk {
        &mut self.stack.last_mut().unwrap().chunk
    }

    fn emit(&mut self, op: Op) -> usize {
        self.chunk().emit(op)
    }

    fn here(&mut self) -> usize {
        self.chunk().code.len()
    }

    fn add_const(&mut self, v: Value) -> usize {
        self.chunk().add_const(v)
    }

    fn patch(&mut self, idx: usize) {
        let target = self.here();
        let new = match &self.chunk().code[idx] {
            Op::Jump(_) => Op::Jump(target),
            Op::JumpIfFalse(_) => Op::JumpIfFalse(target),
            Op::JumpIfFalsePeek(_) => Op::JumpIfFalsePeek(target),
            Op::JumpIfTruePeek(_) => Op::JumpIfTruePeek(target),
            other => other.clone(),
        };
        self.chunk().code[idx] = new;
    }

    fn begin_scope(&mut self) {
        self.cur().scope_depth += 1;
    }

    fn end_scope(&mut self) {
        let fs = self.cur();
        fs.scope_depth -= 1;
        while let Some(l) = fs.locals.last() {
            if l.depth > fs.scope_depth {
                fs.locals.pop();
                fs.chunk.emit(Op::Pop);
            } else {
                break;
            }
        }
    }

    fn resolve_local(&self, name: &str) -> Option<usize> {
        let fs = self.stack.last().unwrap();
        for (i, l) in fs.locals.iter().enumerate().rev() {
            if l.name == name {
                return Some(i);
            }
        }
        None
    }

    /// Declares a name. Returns Some(global_const_index) if it's a global,
    /// or None if it became a local (value left on the stack as its slot).
    fn declare(&mut self, name: &str) -> Option<usize> {
        if self.cur().scope_depth == 0 {
            Some(self.add_const(Value::Str(Rc::new(name.to_string()))))
        } else {
            let depth = self.cur().scope_depth;
            self.cur().locals.push(Local { name: name.to_string(), depth });
            None
        }
    }

    // ---- Statements ----

    fn stmt(&mut self, s: &Stmt) -> Result<(), String> {
        match s {
            Stmt::Let { name, value } => {
                self.expr(value)?;
                match self.declare(name) {
                    Some(g) => {
                        self.emit(Op::DefineGlobal(g));
                    }
                    None => { /* value stays on stack as the local */ }
                }
            }
            Stmt::Assign { name, value } => {
                self.expr(value)?;
                if let Some(slot) = self.resolve_local(name) {
                    self.emit(Op::SetLocal(slot));
                } else {
                    let g = self.add_const(Value::Str(Rc::new(name.clone())));
                    self.emit(Op::SetGlobal(g));
                }
                self.emit(Op::Pop); // assignment is a statement here
            }
            Stmt::ExprStmt(e) => {
                self.expr(e)?;
                self.emit(Op::Pop);
            }
            Stmt::If { cond, then, els } => {
                self.expr(cond)?;
                let jf = self.emit(Op::JumpIfFalse(0));
                self.begin_scope();
                for st in then {
                    self.stmt(st)?;
                }
                self.end_scope();
                let jend = self.emit(Op::Jump(0));
                self.patch(jf);
                self.begin_scope();
                for st in els {
                    self.stmt(st)?;
                }
                self.end_scope();
                self.patch(jend);
            }
            Stmt::While { cond, body } => {
                let start = self.here();
                self.expr(cond)?;
                let exit = self.emit(Op::JumpIfFalse(0));
                self.begin_scope();
                for st in body {
                    self.stmt(st)?;
                }
                self.end_scope();
                self.emit(Op::Jump(start));
                self.patch(exit);
            }
            Stmt::For { var, start, end, body } => {
                // desugar: { let var = start; while var < end { body; var = var + 1 } }
                self.begin_scope();
                self.expr(start)?;
                self.declare(var); // local within this scope
                let cond_start = self.here();
                // var < end
                let slot = self.resolve_local(var).expect("for var is local");
                self.emit(Op::GetLocal(slot));
                self.expr(end)?;
                self.emit(Op::Lt);
                let exit = self.emit(Op::JumpIfFalse(0));
                // body
                self.begin_scope();
                for st in body {
                    self.stmt(st)?;
                }
                self.end_scope();
                // var = var + 1
                self.emit(Op::GetLocal(slot));
                let one = self.add_const(Value::Int(1));
                self.emit(Op::Const(one));
                self.emit(Op::Add);
                self.emit(Op::SetLocal(slot));
                self.emit(Op::Pop);
                self.emit(Op::Jump(cond_start));
                self.patch(exit);
                self.end_scope();
            }
            Stmt::Func { name, params, body } => {
                let func = self.compile_callable(name, params, body, false)?;
                let c = self.add_const(Value::Func(Rc::new(func)));
                self.emit(Op::Const(c));
                match self.declare(name) {
                    Some(g) => self.emit(Op::DefineGlobal(g)),
                    None => { /* local function */ 0 }
                };
            }
            Stmt::Class { name, methods } => {
                let name_c = self.add_const(Value::Str(Rc::new(name.clone())));
                self.emit(Op::Class(name_c));
                for m in methods {
                    let is_init = m.name == "init";
                    let func = self.compile_callable(&m.name, &m.params, &m.body, true)?;
                    if is_init {
                        // arity excludes the implicit `self`
                    }
                    let fc = self.add_const(Value::Func(Rc::new(func)));
                    self.emit(Op::Const(fc));
                    let mc = self.add_const(Value::Str(Rc::new(m.name.clone())));
                    self.emit(Op::Method(mc));
                }
                // the class is now on top of the stack
                if let Some(g) = self.declare(name) {
                    self.emit(Op::DefineGlobal(g));
                }
            }
            Stmt::IndexAssign { obj, index, value } => {
                self.expr(obj)?;
                self.expr(index)?;
                self.expr(value)?;
                self.emit(Op::IndexSet);
                self.emit(Op::Pop);
            }
            Stmt::PropAssign { obj, name, value } => {
                self.expr(obj)?;
                self.expr(value)?;
                let nc = self.add_const(Value::Str(Rc::new(name.clone())));
                self.emit(Op::SetProp(nc));
                self.emit(Op::Pop);
            }
            Stmt::Return(opt) => {
                match opt {
                    Some(e) => self.expr(e)?,
                    None => {
                        if self.cur().is_init {
                            self.emit(Op::GetLocal(0)); // init returns self
                        } else {
                            self.emit(Op::Null);
                        }
                    }
                }
                self.emit(Op::Return);
            }
        }
        Ok(())
    }

    fn compile_callable(
        &mut self,
        name: &str,
        params: &[String],
        body: &[Stmt],
        is_method: bool,
    ) -> Result<Function, String> {
        let mut fs = FuncState::new();
        fs.is_init = is_method && name == "init";
        self.stack.push(fs);
        self.begin_scope();
        // slot 0 is reserved: `self` for methods, an unnamed slot (the callee) for functions
        let depth = self.cur().scope_depth;
        let slot0 = if is_method { "self" } else { "" };
        self.cur().locals.push(Local { name: slot0.to_string(), depth });
        for p in params {
            let depth = self.cur().scope_depth;
            self.cur().locals.push(Local { name: p.clone(), depth });
        }
        for st in body {
            self.stmt(st)?;
        }
        // implicit return: self for init, else null
        if self.cur().is_init {
            self.emit(Op::GetLocal(0));
        } else {
            self.emit(Op::Null);
        }
        self.emit(Op::Return);
        let fs = self.stack.pop().unwrap();
        Ok(Function { name: name.to_string(), arity: params.len(), chunk: fs.chunk })
    }

    // ---- Expressions ----

    fn expr(&mut self, e: &Expr) -> Result<(), String> {
        match e {
            Expr::Int(n) => {
                let c = self.add_const(Value::Int(*n));
                self.emit(Op::Const(c));
            }
            Expr::Float(f) => {
                let c = self.add_const(Value::Float(*f));
                self.emit(Op::Const(c));
            }
            Expr::Str(s) => {
                let c = self.add_const(Value::Str(Rc::new(s.clone())));
                self.emit(Op::Const(c));
            }
            Expr::Bool(b) => {
                self.emit(if *b { Op::True } else { Op::False });
            }
            Expr::Null => {
                self.emit(Op::Null);
            }
            Expr::Var(name) => {
                if let Some(slot) = self.resolve_local(name) {
                    self.emit(Op::GetLocal(slot));
                } else {
                    let g = self.add_const(Value::Str(Rc::new(name.clone())));
                    self.emit(Op::GetGlobal(g));
                }
            }
            Expr::Unary { op, expr } => {
                self.expr(expr)?;
                match op {
                    UnOp::Neg => self.emit(Op::Neg),
                    UnOp::Not => self.emit(Op::Not),
                };
            }
            Expr::Binary { op, lhs, rhs } => {
                self.expr(lhs)?;
                self.expr(rhs)?;
                let o = match op {
                    BinOp::Add => Op::Add,
                    BinOp::Sub => Op::Sub,
                    BinOp::Mul => Op::Mul,
                    BinOp::Div => Op::Div,
                    BinOp::Mod => Op::Mod,
                    BinOp::Eq => Op::Eq,
                    BinOp::Ne => Op::Ne,
                    BinOp::Lt => Op::Lt,
                    BinOp::Le => Op::Le,
                    BinOp::Gt => Op::Gt,
                    BinOp::Ge => Op::Ge,
                };
                self.emit(o);
            }
            Expr::Logical { op, lhs, rhs } => {
                self.expr(lhs)?;
                match op {
                    LogOp::And => {
                        let j = self.emit(Op::JumpIfFalsePeek(0));
                        self.emit(Op::Pop);
                        self.expr(rhs)?;
                        self.patch(j);
                    }
                    LogOp::Or => {
                        let j = self.emit(Op::JumpIfTruePeek(0));
                        self.emit(Op::Pop);
                        self.expr(rhs)?;
                        self.patch(j);
                    }
                }
            }
            Expr::Array(elems) => {
                for el in elems {
                    self.expr(el)?;
                }
                self.emit(Op::NewArray(elems.len()));
            }
            Expr::Map(pairs) => {
                for (k, v) in pairs {
                    self.expr(k)?;
                    self.expr(v)?;
                }
                self.emit(Op::NewMap(pairs.len()));
            }
            Expr::Match { subject, arms } => {
                self.expr(subject)?; // [subj]
                let mut end_jumps = Vec::new();
                let mut has_wild = false;
                for (pat, body) in arms {
                    match pat {
                        Some(p) => {
                            self.emit(Op::Dup);
                            self.expr(p)?;
                            self.emit(Op::Eq);
                            let next = self.emit(Op::JumpIfFalse(0)); // pops bool
                            self.emit(Op::Pop); // drop subject
                            self.expr(body)?;
                            end_jumps.push(self.emit(Op::Jump(0)));
                            self.patch(next);
                        }
                        None => {
                            has_wild = true;
                            self.emit(Op::Pop); // drop subject
                            self.expr(body)?;
                            end_jumps.push(self.emit(Op::Jump(0)));
                            break; // wildcard is terminal
                        }
                    }
                }
                if !has_wild {
                    self.emit(Op::Pop); // no arm matched: drop subject
                    self.emit(Op::Null);
                }
                for j in end_jumps {
                    self.patch(j);
                }
            }
            Expr::Index { obj, index } => {
                self.expr(obj)?;
                self.expr(index)?;
                self.emit(Op::IndexGet);
            }
            Expr::Get { obj, name } => {
                self.expr(obj)?;
                let nc = self.add_const(Value::Str(Rc::new(name.clone())));
                self.emit(Op::GetProp(nc));
            }
            Expr::MethodCall { obj, name, args } => {
                self.expr(obj)?; // receiver -> becomes self at the call frame base
                for a in args {
                    self.expr(a)?;
                }
                let nc = self.add_const(Value::Str(Rc::new(name.clone())));
                self.emit(Op::Invoke(nc, args.len()));
            }
            Expr::Call { name, args } => {
                if name == "print" {
                    for a in args {
                        self.expr(a)?;
                    }
                    self.emit(Op::Print(args.len()));
                } else if let Some(id) = native_id(name) {
                    for a in args {
                        self.expr(a)?;
                    }
                    self.emit(Op::Native(id, args.len()));
                } else {
                    // push callee, then args
                    if let Some(slot) = self.resolve_local(name) {
                        self.emit(Op::GetLocal(slot));
                    } else {
                        let g = self.add_const(Value::Str(Rc::new(name.clone())));
                        self.emit(Op::GetGlobal(g));
                    }
                    for a in args {
                        self.expr(a)?;
                    }
                    self.emit(Op::Call(args.len()));
                }
            }
        }
        Ok(())
    }
}
