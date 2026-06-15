//! Recursive-descent + Pratt parser: tokens -> AST.

use crate::ast::*;
use crate::token::{Tok, Token};

pub struct Parser {
    toks: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(toks: Vec<Token>) -> Self {
        Parser { toks, pos: 0 }
    }

    fn peek(&self) -> &Tok {
        &self.toks[self.pos].tok
    }

    fn line(&self) -> usize {
        self.toks[self.pos].line
    }

    fn advance(&mut self) -> Tok {
        let t = self.toks[self.pos].tok.clone();
        if self.pos < self.toks.len() - 1 {
            self.pos += 1;
        }
        t
    }

    fn check(&self, t: &Tok) -> bool {
        self.peek() == t
    }

    fn accept(&mut self, t: &Tok) -> bool {
        if self.check(t) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, t: &Tok, what: &str) -> Result<(), String> {
        if self.check(t) {
            self.advance();
            Ok(())
        } else {
            Err(format!("line {}: expected {}, found {:?}", self.line(), what, self.peek()))
        }
    }

    pub fn parse(mut self) -> Result<Program, String> {
        let mut stmts = Vec::new();
        while !self.check(&Tok::Eof) {
            stmts.push(self.statement()?);
        }
        Ok(Program { stmts })
    }

    fn block(&mut self) -> Result<Vec<Stmt>, String> {
        self.expect(&Tok::LBrace, "'{'")?;
        let mut stmts = Vec::new();
        while !self.check(&Tok::RBrace) && !self.check(&Tok::Eof) {
            stmts.push(self.statement()?);
        }
        self.expect(&Tok::RBrace, "'}'")?;
        Ok(stmts)
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        match self.peek().clone() {
            Tok::Let | Tok::Const => self.let_stmt(),
            Tok::Func => self.func_stmt(),
            Tok::If => self.if_stmt(),
            Tok::While => self.while_stmt(),
            Tok::For => self.for_stmt(),
            Tok::Return => self.return_stmt(),
            _ => self.expr_or_assign(),
        }
    }

    fn let_stmt(&mut self) -> Result<Stmt, String> {
        self.advance(); // let / const
        let name = self.ident_name("variable name")?;
        // optional type annotation: ": Type" (ignored)
        if self.accept(&Tok::Colon) {
            let _ = self.ident_name("type name")?;
        }
        self.expect(&Tok::Assign, "'='")?;
        let value = self.expression()?;
        self.accept(&Tok::Semicolon);
        Ok(Stmt::Let { name, value })
    }

    fn func_stmt(&mut self) -> Result<Stmt, String> {
        self.advance(); // func
        let name = self.ident_name("function name")?;
        self.expect(&Tok::LParen, "'('")?;
        let mut params = Vec::new();
        if !self.check(&Tok::RParen) {
            loop {
                let p = self.ident_name("parameter name")?;
                if self.accept(&Tok::Colon) {
                    let _ = self.ident_name("type name")?;
                }
                params.push(p);
                if !self.accept(&Tok::Comma) {
                    break;
                }
            }
        }
        self.expect(&Tok::RParen, "')'")?;
        // optional return type: "-> Type" (ignored)
        if self.accept(&Tok::Arrow) {
            let _ = self.ident_name("return type")?;
        }
        let body = self.block()?;
        Ok(Stmt::Func { name, params, body })
    }

    fn if_stmt(&mut self) -> Result<Stmt, String> {
        self.advance(); // if
        let cond = self.expression()?;
        let then = self.block()?;
        let els = if self.accept(&Tok::Else) {
            if self.check(&Tok::If) {
                vec![self.if_stmt()?]
            } else {
                self.block()?
            }
        } else {
            Vec::new()
        };
        Ok(Stmt::If { cond, then, els })
    }

    fn while_stmt(&mut self) -> Result<Stmt, String> {
        self.advance(); // while
        let cond = self.expression()?;
        let body = self.block()?;
        Ok(Stmt::While { cond, body })
    }

    fn for_stmt(&mut self) -> Result<Stmt, String> {
        self.advance(); // for
        let var = self.ident_name("loop variable")?;
        self.expect(&Tok::In, "'in'")?;
        let start = self.expression()?;
        self.expect(&Tok::DotDot, "'..'")?;
        let end = self.expression()?;
        let body = self.block()?;
        Ok(Stmt::For { var, start, end, body })
    }

    fn return_stmt(&mut self) -> Result<Stmt, String> {
        self.advance(); // return
        if self.check(&Tok::Semicolon) || self.check(&Tok::RBrace) {
            self.accept(&Tok::Semicolon);
            return Ok(Stmt::Return(None));
        }
        let e = self.expression()?;
        self.accept(&Tok::Semicolon);
        Ok(Stmt::Return(Some(e)))
    }

    fn expr_or_assign(&mut self) -> Result<Stmt, String> {
        // assignment: IDENT '=' expr  (but not '==')
        if let Tok::Ident(name) = self.peek().clone() {
            if self.toks[self.pos + 1].tok == Tok::Assign {
                self.advance(); // ident
                self.advance(); // =
                let value = self.expression()?;
                self.accept(&Tok::Semicolon);
                return Ok(Stmt::Assign { name, value });
            }
        }
        let e = self.expression()?;
        self.accept(&Tok::Semicolon);
        Ok(Stmt::ExprStmt(e))
    }

    fn ident_name(&mut self, what: &str) -> Result<String, String> {
        match self.advance() {
            Tok::Ident(s) => Ok(s),
            other => Err(format!("line {}: expected {}, found {:?}", self.line(), what, other)),
        }
    }

    // ---- Expressions (precedence climbing) ----

    fn expression(&mut self) -> Result<Expr, String> {
        self.or_expr()
    }

    fn or_expr(&mut self) -> Result<Expr, String> {
        let mut lhs = self.and_expr()?;
        while self.accept(&Tok::Or) {
            let rhs = self.and_expr()?;
            lhs = Expr::Logical { op: LogOp::Or, lhs: Box::new(lhs), rhs: Box::new(rhs) };
        }
        Ok(lhs)
    }

    fn and_expr(&mut self) -> Result<Expr, String> {
        let mut lhs = self.equality()?;
        while self.accept(&Tok::And) {
            let rhs = self.equality()?;
            lhs = Expr::Logical { op: LogOp::And, lhs: Box::new(lhs), rhs: Box::new(rhs) };
        }
        Ok(lhs)
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut lhs = self.comparison()?;
        loop {
            let op = match self.peek() {
                Tok::Eq => BinOp::Eq,
                Tok::Ne => BinOp::Ne,
                _ => break,
            };
            self.advance();
            let rhs = self.comparison()?;
            lhs = Expr::Binary { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
        }
        Ok(lhs)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut lhs = self.term()?;
        loop {
            let op = match self.peek() {
                Tok::Lt => BinOp::Lt,
                Tok::Le => BinOp::Le,
                Tok::Gt => BinOp::Gt,
                Tok::Ge => BinOp::Ge,
                _ => break,
            };
            self.advance();
            let rhs = self.term()?;
            lhs = Expr::Binary { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
        }
        Ok(lhs)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut lhs = self.factor()?;
        loop {
            let op = match self.peek() {
                Tok::Plus => BinOp::Add,
                Tok::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let rhs = self.factor()?;
            lhs = Expr::Binary { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
        }
        Ok(lhs)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut lhs = self.unary()?;
        loop {
            let op = match self.peek() {
                Tok::Star => BinOp::Mul,
                Tok::Slash => BinOp::Div,
                Tok::Percent => BinOp::Mod,
                _ => break,
            };
            self.advance();
            let rhs = self.unary()?;
            lhs = Expr::Binary { op, lhs: Box::new(lhs), rhs: Box::new(rhs) };
        }
        Ok(lhs)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            Tok::Minus => {
                self.advance();
                Ok(Expr::Unary { op: UnOp::Neg, expr: Box::new(self.unary()?) })
            }
            Tok::Bang => {
                self.advance();
                Ok(Expr::Unary { op: UnOp::Not, expr: Box::new(self.unary()?) })
            }
            _ => self.primary(),
        }
    }

    fn primary(&mut self) -> Result<Expr, String> {
        match self.advance() {
            Tok::Int(n) => Ok(Expr::Int(n)),
            Tok::Float(f) => Ok(Expr::Float(f)),
            Tok::Str(s) => Ok(Expr::Str(s)),
            Tok::True => Ok(Expr::Bool(true)),
            Tok::False => Ok(Expr::Bool(false)),
            Tok::Null => Ok(Expr::Null),
            Tok::LParen => {
                let e = self.expression()?;
                self.expect(&Tok::RParen, "')'")?;
                Ok(e)
            }
            Tok::Ident(name) => {
                if self.accept(&Tok::LParen) {
                    let mut args = Vec::new();
                    if !self.check(&Tok::RParen) {
                        loop {
                            args.push(self.expression()?);
                            if !self.accept(&Tok::Comma) {
                                break;
                            }
                        }
                    }
                    self.expect(&Tok::RParen, "')'")?;
                    Ok(Expr::Call { name, args })
                } else {
                    Ok(Expr::Var(name))
                }
            }
            other => Err(format!("line {}: unexpected token {:?}", self.line(), other)),
        }
    }
}
