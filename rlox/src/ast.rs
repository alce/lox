#![allow(unused)]
use std::fmt;

use crate::visitor::AstPrinter;

// Expressions
#[derive(Debug)]
pub enum Expr {
    Binary {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Literal(Lit),
}

// Literals
#[derive(Clone, PartialEq, Debug)]
pub enum Lit {
    Str(String),
    Num(f64),
    Bool(bool),
    Nil,
}

// Binary Operators
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
}

// Unary Operators
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum UnOp {
    Neg,
    Not,
}

impl Expr {
    pub fn binary(lhs: Expr, op: BinOp, rhs: Expr) -> Expr {
        Expr::Binary {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }

    pub fn unary(op: UnOp, rhs: Expr) -> Expr {
        Expr::Unary(op, Box::new(rhs))
    }

    pub fn grouping(expr: Expr) -> Expr {
        Expr::Grouping(Box::new(expr))
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", AstPrinter.print(self))
    }
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lit::Str(s) => f.write_str(&s),
            Lit::Num(n) => write!(f, "{:.1}", n),
            Lit::Bool(b) => write!(f, "{}", b),
            Lit::Nil => f.write_str("nil"),
        }
    }
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Eq => "==",
            BinOp::NotEq => "!=",
            BinOp::Lt => "<",
            BinOp::LtEq => "<=",
            BinOp::Gt => ">",
            BinOp::GtEq => ">=",
        })
    }
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            UnOp::Neg => "-",
            UnOp::Not => "!",
        })
    }
}
