//! Abstract Syntax Tree for VyroLang.

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    Null,
    Var(String),
    Array(Vec<Expr>),
    Map(Vec<(Expr, Expr)>),
    Match { subject: Box<Expr>, arms: Vec<(Option<Expr>, Expr)> },
    Index { obj: Box<Expr>, index: Box<Expr> },
    Get { obj: Box<Expr>, name: String },
    Unary { op: UnOp, expr: Box<Expr> },
    Binary { op: BinOp, lhs: Box<Expr>, rhs: Box<Expr> },
    Logical { op: LogOp, lhs: Box<Expr>, rhs: Box<Expr> },
    Call { name: String, args: Vec<Expr> },
    MethodCall { obj: Box<Expr>, name: String, args: Vec<Expr> },
}

#[derive(Debug, Clone, Copy)]
pub enum UnOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone, Copy)]
pub enum LogOp {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub struct Method {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: String, value: Expr },
    Assign { name: String, value: Expr },
    IndexAssign { obj: Expr, index: Expr, value: Expr },
    PropAssign { obj: Expr, name: String, value: Expr },
    ExprStmt(Expr),
    If { cond: Expr, then: Vec<Stmt>, els: Vec<Stmt> },
    While { cond: Expr, body: Vec<Stmt> },
    For { var: String, start: Expr, end: Expr, body: Vec<Stmt> },
    Func { name: String, params: Vec<String>, body: Vec<Stmt> },
    Class { name: String, methods: Vec<Method> },
    Return(Option<Expr>),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}
