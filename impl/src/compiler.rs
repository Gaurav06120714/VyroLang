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
}

impl FuncState {
    fn new() -> Self {
        FuncState { chunk: Chunk::new(), locals: Vec::new(), scope_depth: 0 }
    }
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
                let func = self.compile_function(name, params, body)?;
                let c = self.add_const(Value::Func(Rc::new(func)));
                self.emit(Op::Const(c));
                match self.declare(name) {
                    Some(g) => self.emit(Op::DefineGlobal(g)),
                    None => { /* local function */ 0 }
                };
            }
            Stmt::Return(opt) => {
                match opt {
                    Some(e) => self.expr(e)?,
                    None => {
                        self.emit(Op::Null);
                    }
                }
                self.emit(Op::Return);
            }
        }
        Ok(())
    }

    fn compile_function(
        &mut self,
        name: &str,
        params: &[String],
        body: &[Stmt],
    ) -> Result<Function, String> {
        self.stack.push(FuncState::new());
        self.begin_scope();
        for p in params {
            let depth = self.cur().scope_depth;
            self.cur().locals.push(Local { name: p.clone(), depth });
        }
        for st in body {
            self.stmt(st)?;
        }
        // implicit return null
        self.emit(Op::Null);
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
            Expr::Call { name, args } => {
                if name == "print" {
                    for a in args {
                        self.expr(a)?;
                    }
                    self.emit(Op::Print(args.len()));
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
