use crate::ast::{BinOp, Expr, UnOp};
use crate::value::Value;
use crate::visitor::Visitor;

pub struct Interpreter {}

impl Interpreter {
    pub fn interpret(&mut self, expr: &Expr) -> Value {
        self.evaluate(expr).unwrap()
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        self.visit_expr(expr)
    }

    fn unary_expr(&mut self, op: UnOp, rhs: &Expr) -> Result<Value, String> {
        let right: Value = self.evaluate(rhs)?;

        match op {
            UnOp::Neg => -right,
            UnOp::Not => !right,
        }
    }

    fn binary_expr(&mut self, lhs: &Expr, op: BinOp, rhs: &Expr) -> Result<Value, String> {
        let lhs = self.evaluate(lhs)?;
        let rhs = self.evaluate(rhs)?;

        match op {
            BinOp::Sub => return lhs - rhs,
            BinOp::Div => return lhs / rhs,
            BinOp::Mul => return lhs * rhs,
            BinOp::Add => return lhs + rhs,
            _ => {}
        }

        Ok(Value::from(match op {
            BinOp::Gt => lhs > rhs,
            BinOp::GtEq => lhs >= rhs,
            BinOp::Lt => lhs < rhs,
            BinOp::LtEq => lhs <= rhs,
            BinOp::Eq => lhs == rhs,
            BinOp::NotEq => lhs != rhs,
            _ => unreachable!(),
        }))
    }
}

impl Visitor<Result<Value, String>> for Interpreter {
    fn visit_expr(&mut self, e: &Expr) -> Result<Value, String> {
        match e {
            Expr::Literal(lit) => Ok(Value::from(lit)),
            Expr::Grouping(expr) => self.visit_expr(expr),
            Expr::Unary(op, expr) => self.unary_expr(*op, expr),
            Expr::Binary { lhs, op, rhs } => self.binary_expr(lhs, *op, rhs),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    #[test]
    fn arithmetic() {
        let expr = parse("(5 - (3 - 1)) + -1");
        let mut interpreter = Interpreter {};
        let val = interpreter.interpret(&expr);

        assert_eq!(val.to_string(), "2")
    }
}
