use std::fmt;

use crate::token::TokenKind;
use crate::visitor::AstPrinter;

// Binary Operators
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BinOp {
    Add,
    Div,
    Eq,
    Gt,
    GtEq,
    Lt,
    LtEq,
    Mul,
    NotEq,
    Sub,
}

// Expressions
#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Assign(String, Box<Expr>, u64),
    Binary {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
        line: u64,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        line: u64,
    },
    Grouping(Box<Expr>),
    Unary(UnOp, Box<Expr>, u64),
    Literal(Lit),
    Logical {
        lhs: Box<Expr>,
        kw: Keyword,
        rhs: Box<Expr>,
        line: u64,
    },
    Variable(String, u64),
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Keyword {
    And,
    Or,
}

// Literals
#[derive(Clone, PartialEq, Debug)]
pub enum Lit {
    Bool(bool),
    Nil,
    Num(f64),
    Str(String),
}

// Statements
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expr(Expr),
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then: Box<Stmt>,
        r#else: Option<Box<Stmt>>,
    },
    Function {
        name: String,
        // parameter names
        params: Vec<String>,
        body: Vec<Stmt>,
        line: u64,
    },
    Print(Expr),
    Return(Option<Expr>, u64),
    Var(String, Option<Expr>),
    While(Expr, Box<Stmt>),
}

// Unary Operators
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum UnOp {
    Neg,
    Not,
}

impl Expr {
    pub fn assign(name: String, expr: Expr, line: u64) -> Expr {
        Expr::Assign(name, Box::new(expr), line)
    }

    pub fn binary(lhs: Expr, op: BinOp, rhs: Expr, line: u64) -> Expr {
        Expr::Binary {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
            line,
        }
    }

    pub fn call(callee: Expr, args: Vec<Expr>, line: u64) -> Expr {
        Expr::Call {
            callee: Box::new(callee),
            args,
            line,
        }
    }

    pub fn grouping(expr: Expr) -> Expr {
        Expr::Grouping(Box::new(expr))
    }

    pub fn logical(lhs: Expr, kw: Keyword, rhs: Expr, line: u64) -> Expr {
        Expr::Logical {
            lhs: Box::new(lhs),
            kw,
            rhs: Box::new(rhs),
            line,
        }
    }

    pub fn unary(op: UnOp, rhs: Expr, line: u64) -> Expr {
        Expr::Unary(op, Box::new(rhs), line)
    }
}

impl From<TokenKind<'_>> for BinOp {
    fn from(t: TokenKind<'_>) -> Self {
        match t {
            TokenKind::BANG_EQUAL => BinOp::NotEq,
            TokenKind::EQUAL_EQUAL => BinOp::Eq,
            TokenKind::GREATER => BinOp::Gt,
            TokenKind::GREATER_EQUAL => BinOp::GtEq,
            TokenKind::LESS => BinOp::Lt,
            TokenKind::LESS_EQUAL => BinOp::LtEq,
            TokenKind::MINUS => BinOp::Sub,
            TokenKind::PLUS => BinOp::Add,
            TokenKind::SLASH => BinOp::Div,
            TokenKind::STAR => BinOp::Mul,
            _ => panic!("{} is not a binary operator", t),
        }
    }
}

impl From<TokenKind<'_>> for Keyword {
    fn from(t: TokenKind<'_>) -> Self {
        match t {
            TokenKind::AND => Keyword::And,
            TokenKind::OR => Keyword::Or,
            _ => panic!("{} is not a keyword", t),
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

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            BinOp::Add => "+",
            BinOp::Div => "/",
            BinOp::Eq => "==",
            BinOp::Gt => ">",
            BinOp::GtEq => ">=",
            BinOp::Lt => "<",
            BinOp::LtEq => "<=",
            BinOp::Mul => "*",
            BinOp::NotEq => "!=",
            BinOp::Sub => "-",
        })
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", AstPrinter.print(self))
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Keyword::And => "and",
            Keyword::Or => "or",
        })
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

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            UnOp::Neg => "-",
            UnOp::Not => "!",
        })
    }
}
