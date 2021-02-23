use crate::ast::{BinOp, Expr, Lit, UnOp};
use crate::token::{Token, TokenKind};
use TokenKind::*;

#[allow(unused)]
pub enum ParseError {
    EndOfStream,
    UnexpectedToken,
}

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    idx: usize,
}

pub fn parse(source: &str) -> Expr {
    let tokens = crate::scanner::tokenize(source).collect();
    Parser::new(tokens).parse().unwrap()
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser { tokens, idx: 0 }
    }

    pub fn parse(&mut self) -> Option<Expr> {
        Some(self.expression())
    }

    fn expression(&mut self) -> Expr {
        self.expr_bp(0)
    }

    fn expr_bp(&mut self, min_bp: u8) -> Expr {
        let mut lhs = match self.consume() {
            NUMBER(n) => Expr::Literal(Lit::Num(n)),
            LEFT_PAREN => {
                let lhs = self.expr_bp(0);
                assert_eq!(self.consume(), RIGHT_PAREN);
                Expr::grouping(lhs)
            }
            t @ MINUS | t @ PLUS => {
                let ((), rbp) = prefix_bp(t);
                let rhs = self.expr_bp(rbp);
                Expr::unary(un_op(t), rhs)
            }
            t => panic!("bad token: {:?}", t),
        };

        loop {
            let op = match self.peek() {
                EOF => break,
                t if t.is_operator() => t,
                o => panic!("bad token: {:?}", o),
            };

            if let Some((lbp, rbp)) = infix_bp(op) {
                if lbp < min_bp {
                    break;
                }
                self.consume();
                let rhs = self.expr_bp(rbp);
                lhs = Expr::binary(lhs, bin_op(op), rhs);

                continue;
            }
            break;
        }
        lhs
    }

    fn consume(&mut self) -> TokenKind<'a> {
        self.idx += 1;
        self.tokens[self.idx - 1].kind
    }

    fn peek(&self) -> TokenKind<'a> {
        match self.tokens.get(self.idx) {
            Some(t) => t.kind,
            None => TokenKind::EOF,
        }
    }
}

fn bin_op(t: TokenKind<'_>) -> BinOp {
    match t {
        PLUS => BinOp::Add,
        MINUS => BinOp::Sub,
        STAR => BinOp::Mul,
        SLASH => BinOp::Div,
        _ => panic!("not a BinOp {:?}", t),
    }
}

fn un_op(t: TokenKind<'_>) -> UnOp {
    match t {
        BANG => UnOp::Neg,
        MINUS => UnOp::Neg,
        _ => panic!("not an UnOp {:?}", t),
    }
}

fn prefix_bp(t: TokenKind<'_>) -> ((), u8) {
    match t {
        PLUS | MINUS => ((), 5),
        _ => panic!("bad op: {:?}", t),
    }
}

fn infix_bp(t: TokenKind<'_>) -> Option<(u8, u8)> {
    let res = match t {
        PLUS | MINUS => (1, 2),
        STAR | SLASH => (3, 4),
        _ => return None,
    };

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic_expression() {
        let expr = parse("(5 - (3 - 1)) + -1");
        let expected = "(+ (group (- 5.0 (group (- 3.0 1.0)))) (- 1.0))";

        assert_eq!(expr.to_string(), expected);
    }
}
