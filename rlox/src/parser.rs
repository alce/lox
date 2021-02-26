use crate::ast::{Expr, Lit, Stmt};
use crate::token::{Token, TokenKind};
use crate::LoxError;
use TokenKind::*;

type Result<T> = std::result::Result<T, LoxError>;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    idx: usize,
}

pub fn parse(source: &str) -> Result<Vec<Stmt>> {
    let tokens = crate::scanner::tokenize(source).collect();
    Parser::new(tokens).parse()
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser { tokens, idx: 0 }
    }

    fn parse(&mut self) -> Result<Vec<Stmt>> {
        let mut stmts = vec![];

        while !self.at_end() {
            stmts.push(self.declaration()?)
        }

        Ok(stmts)
    }

    fn expression(&mut self) -> Result<Expr> {
        self.assignment()
    }

    fn block(&mut self) -> Result<Vec<Stmt>> {
        let mut stmts = vec![];

        while !self.check(RIGHT_BRACE) && !self.at_end() {
            stmts.push(self.declaration()?)
        }

        self.consume(RIGHT_BRACE, "Expect '}' after block.")?;

        Ok(stmts)
    }

    fn declaration(&mut self) -> Result<Stmt> {
        if self._match(&[VAR]) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn statement(&mut self) -> Result<Stmt> {
        match self.peek().kind {
            PRINT => {
                self.advance();
                self.print_statement()
            }
            LEFT_BRACE => {
                self.advance();
                Ok(Stmt::Block(self.block()?))
            }
            IF => {
                self.advance();
                self.if_statement()
            }
            _ => self.expression_statement(),
        }
    }

    fn if_statement(&mut self) -> Result<Stmt> {
        self.consume(LEFT_PAREN, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(RIGHT_PAREN, "Expect ')' after 'if condition'.")?;

        let then = self.statement()?;
        let r#else = if self._match(&[ELSE]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            then: Box::new(then),
            r#else,
        })
    }

    fn print_statement(&mut self) -> Result<Stmt> {
        let val = self.expression()?;
        self.consume(SEMICOLON, "Expect ';' after value.")?;
        Ok(Stmt::Print(val))
    }

    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self.consume_ident("Expect variable name.")?;

        let initializer = if self._match(&[EQUAL]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(SEMICOLON, "Expect ';' after variable declaration.")?;
        Ok(Stmt::Var(name.to_string(), initializer))
    }

    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(SEMICOLON, "Expect ';' after expression.")?;
        Ok(Stmt::Expr(expr))
    }

    fn assignment(&mut self) -> Result<Expr> {
        let expr = self.or()?;

        if self._match(&[EQUAL]) {
            let tok = self.previous();
            let val = self.assignment()?;

            return match expr {
                Expr::Variable(name, ..) => Ok(Expr::assign(name, val, self.peek().line)),
                _ => Err(self.parse_error(tok, "Invalid assignment target.")),
            };
        }

        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr> {
        let mut lhs = self.and()?;

        while self._match(&[OR]) {
            let op = self.previous();
            let rhs = self.and()?;
            lhs = Expr::logical(lhs, op.kind.into(), rhs, op.line);
        }

        Ok(lhs)
    }

    fn and(&mut self) -> Result<Expr> {
        let mut lhs = self.equality()?;

        while self._match(&[AND]) {
            let op = self.previous();
            let rhs = self.equality()?;
            lhs = Expr::logical(lhs, op.kind.into(), rhs, op.line);
        }

        Ok(lhs)
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut lhs = self.comparison()?;

        while self._match(&[BANG_EQUAL, EQUAL_EQUAL]) {
            let op = self.previous();
            let rhs = self.comparison()?;
            lhs = Expr::binary(lhs, op.kind.into(), rhs, op.line)
        }

        Ok(lhs)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let mut lhs = self.term()?;

        while self._match(&[GREATER, GREATER_EQUAL, LESS, LESS_EQUAL]) {
            let op = self.previous();
            let rhs = self.term()?;
            lhs = Expr::binary(lhs, op.kind.into(), rhs, op.line);
        }

        Ok(lhs)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut lhs = self.factor()?;

        while self._match(&[MINUS, PLUS]) {
            let op = self.previous();
            let rhs = self.factor()?;
            lhs = Expr::binary(lhs, op.kind.into(), rhs, op.line);
        }

        Ok(lhs)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut lhs = self.unary()?;

        while self._match(&[SLASH, STAR]) {
            let op = self.previous();
            let rhs = self.unary()?;
            lhs = Expr::binary(lhs, op.kind.into(), rhs, op.line);
        }

        Ok(lhs)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self._match(&[BANG, MINUS]) {
            let op = self.previous();
            let right = self.unary()?;
            return Ok(Expr::unary(op.kind.into(), right, op.line));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr> {
        match self.advance().kind {
            FALSE => Ok(Expr::Literal(Lit::Bool(false))),
            TRUE => Ok(Expr::Literal(Lit::Bool(true))),
            NIL => Ok(Expr::Literal(Lit::Nil)),
            NUMBER(n) => Ok(Expr::Literal(Lit::Num(n))),
            STRING(s) => Ok(Expr::Literal(Lit::Str(s.to_string()))),
            LEFT_PAREN => {
                let expr = self.expression()?;
                self.consume(RIGHT_PAREN, "Expect ')' after expression.")?;
                Ok(Expr::grouping(expr))
            }
            IDENTIFIER(s) => Ok(Expr::Variable(s.to_string(), self.peek().line)),
            _ => Err(self.parse_error(self.previous(), "Expect expression.")),
        }
    }

    #[allow(unused)]
    fn synchronize(&mut self) {
        self.advance();

        while !self.at_end() {
            if self.previous().kind == SEMICOLON {
                return;
            }

            match self.peek().kind {
                CLASS | FUN | VAR | FOR | IF | WHILE | PRINT | RETURN => return,
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn _match(&mut self, kinds: &[TokenKind<'a>]) -> bool {
        for kind in kinds {
            if self.check(*kind) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&self, kind: TokenKind<'a>) -> bool {
        if self.at_end() {
            return false;
        }

        self.peek().kind == kind
    }

    fn advance(&mut self) -> Token<'a> {
        if !self.at_end() {
            self.idx += 1;
        }
        self.previous()
    }

    fn at_end(&self) -> bool {
        self.peek().kind == EOF
    }

    fn peek(&self) -> Token<'a> {
        self.tokens[self.idx]
    }

    fn previous(&self) -> Token<'a> {
        self.tokens[self.idx - 1]
    }

    fn consume(&mut self, kind: TokenKind<'a>, msg: &str) -> Result<Token<'a>> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(self.parse_error(self.peek(), msg))
        }
    }

    fn consume_ident(&mut self, msg: &str) -> Result<Token<'a>> {
        match self.peek().kind {
            IDENTIFIER(_) => Ok(self.advance()),
            _ => Err(self.parse_error(self.peek(), msg)),
        }
    }

    fn parse_error(&self, token: Token<'a>, msg: &str) -> LoxError {
        let mut s = format!("[line {}] Error", token.line);

        match token.kind {
            EOF => s = format!("{} at end {}", s, msg),
            ERROR(msg) => s = format!("{}: {}", s, msg),
            _ => s = format!("{} at '{}': {}", s, token, msg),
        }

        LoxError::Compile(s)
    }
}
