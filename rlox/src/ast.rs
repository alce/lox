use std::fmt;

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

pub enum UnOp {
    Neg,
    Not,
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            UnOp::Neg => "-",
            UnOp::Not => "!",
        })
    }
}

pub enum Lit {
    Str(String),
    Num(f64),
    Bool(bool),
    Nil,
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Lit::Str(s) => f.write_str(&s),
            Lit::Num(n) => write!(f, "{}", n),
            Lit::Bool(b) => write!(f, "{}", b),
            Lit::Nil => f.write_str("nil"),
        }
    }
}

pub enum Expr {
    Binary {
        lhs: Box<Expr>,
        op: BinOp,
        rhs: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Unary(UnOp, Box<Expr>),
    Lit(Lit),
}

pub trait Visitor<T> {
    fn visit_expr(&mut self, e: &Expr) -> T;
}

struct AstPrinter;

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        self.visit_expr(expr)
    }

    fn parenthesize(&mut self, name: &impl fmt::Display, expr: &[&Expr]) -> String {
        let ex = expr.iter().fold(String::new(), |mut s, e| {
            s.push_str(&self.visit_expr(e));
            s
        });
        format!("({} {})", name, ex)
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_expr(&mut self, e: &Expr) -> String {
        match e {
            Expr::Binary { rhs, lhs, op } => self.parenthesize(op, &[lhs, rhs]),
            Expr::Grouping(exp) => self.parenthesize(&"group", &[exp]),
            Expr::Unary(op, expr) => self.parenthesize(op, &[expr]),
            Expr::Lit(lit) => format!("{}", lit),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printer() {
        let exp = Expr::Binary {
            lhs: Box::new(Expr::Unary(UnOp::Neg, Box::new(Expr::Lit(Lit::Num(123.0))))),
            op: BinOp::Mul,
            rhs: Box::new(Expr::Grouping(Box::new(Expr::Lit(Lit::Num(45.67))))),
        };

        let mut printer = AstPrinter;
        let res = printer.print(&exp);

        assert_eq!(res, "(* (- 123)(group 45.67))");
    }
}
