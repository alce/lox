use std::convert::TryFrom;
use std::fmt;

use crate::token::TokenKind;
use crate::visitor::AstPrinter;

// Expressions
#[derive(Debug)]
pub enum Expr {
    Assign(String, Box<Expr>, u64),
    Binary {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
        line: u64,
    },
    Grouping(Box<Expr>),
    Unary(UnOp, Box<Expr>, u64),
    Literal(Lit),
    Variable(String, u64),
}

// Statements
#[derive(Debug)]
pub enum Stmt {
    Expr(Expr),
    Print(Expr),
    Var(String, Option<Expr>),
    Block(Vec<Stmt>),
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
    pub fn binary(lhs: Expr, op: BinOp, rhs: Expr, line: u64) -> Expr {
        Expr::Binary {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
            line,
        }
    }

    pub fn unary(op: UnOp, rhs: Expr, line: u64) -> Expr {
        Expr::Unary(op, Box::new(rhs), line)
    }

    pub fn grouping(expr: Expr) -> Expr {
        Expr::Grouping(Box::new(expr))
    }

    pub fn assign(name: String, expr: Expr, line: u64) -> Expr {
        Expr::Assign(name, Box::new(expr), line)
    }
}

impl TryFrom<TokenKind<'_>> for Expr {
    type Error = ();

    fn try_from(t: TokenKind<'_>) -> Result<Self, Self::Error> {
        match t {
            TokenKind::TRUE => Ok(Expr::Literal(Lit::Bool(true))),
            TokenKind::FALSE => Ok(Expr::Literal(Lit::Bool(false))),
            TokenKind::NIL => Ok(Expr::Literal(Lit::Nil)),
            TokenKind::NUMBER(n) => Ok(Expr::Literal(Lit::Num(n))),
            TokenKind::STRING(s) => Ok(Expr::Literal(Lit::Str(s.into()))),
            _ => Err(()),
        }
    }
}

impl From<TokenKind<'_>> for BinOp {
    fn from(t: TokenKind<'_>) -> Self {
        match t {
            TokenKind::PLUS => BinOp::Add,
            TokenKind::MINUS => BinOp::Sub,
            TokenKind::STAR => BinOp::Mul,
            TokenKind::SLASH => BinOp::Div,
            TokenKind::EQUAL_EQUAL => BinOp::Eq,
            TokenKind::BANG_EQUAL => BinOp::NotEq,
            TokenKind::LESS => BinOp::Lt,
            TokenKind::LESS_EQUAL => BinOp::LtEq,
            TokenKind::GREATER => BinOp::Gt,
            TokenKind::GREATER_EQUAL => BinOp::GtEq,
            _ => panic!("{} is not a binary operator", t),
        }
    }
}

impl From<TokenKind<'_>> for UnOp {
    fn from(t: TokenKind<'_>) -> Self {
        match t {
            TokenKind::BANG => UnOp::Not,
            TokenKind::MINUS => UnOp::Neg,
            _ => panic!("{} is not a unary operator", t),
        }
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

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Stmt::Expr(e) => write!(f, "SE({})", e),
            Stmt::Print(e) => write!(f, "Print({})", e),
            Stmt::Var(s, e) => write!(f, "Var({}={:?})", s, e),
            Stmt::Block(stmts) => write!(f, "Block({:?})", stmts),
        }
    }
}
