import 'exception.dart';
import 'expression.dart';
import 'nil.dart';
import 'statement.dart';
import 'token.dart';

class Parser {
  final List<Token> _tokens;

  int _idx = 0;

  Parser(this._tokens);

  bool get _isAtEnd => _peek().type == TokenType.EOF;

  List<Stmt> parse() {
    final statements = <Stmt>[];
    while (!_isAtEnd) {
      statements.add(_declaration());
    }

    return statements;
  }

  // declaration → varDecl | statement ;
  Stmt _declaration() {
    try {
      if (_match([TokenType.VAR])) return _varDeclaration();
      return _statement();
    } on ParseError catch (_) {
      _synchronize();
      rethrow;
    }
  }

  Stmt _varDeclaration() {
    final name = _consume(TokenType.IDENT, 'Expect variable name.');
    var initializer;

    if (_match([TokenType.EQUAL])) {
      initializer = _expression();
    }

    _consume(TokenType.SEMICOLON, "Expect ';' after variable declaration.");
    return VarStmt(name, initializer);
  }

  // statement → exprStmt | printStmt ;
  Stmt _statement() {
    if (_match([TokenType.PRINT])) {
      return _printStatement();
    }

    return _expressionStatement();
  }

  // printStmt → "print" expression ";" ;
  Stmt _printStatement() {
    final value = _expression();
    _consume(TokenType.SEMICOLON, "Expect ';' after value.");
    return PrintStmt(value);
  }

  // exprStmt → expression ";" ;
  Stmt _expressionStatement() {
    final expr = _expression();
    _consume(TokenType.SEMICOLON, "Expect ';' after expression.");
    return ExpressionStmt(expr);
  }

  // expression → equality ;
  Expr _expression() {
    return _assignment();
  }

  Expr _assignment() {
    final expr = _equality();

    if (_match([TokenType.EQUAL])) {
      final equals = _previous();
      final value = _assignment();

      if (expr is VariableExpr) {
        final name = expr.name;
        return AssignExpr(name, value);
      }

      _error(equals, 'Invalid assignment target.');
    }

    return expr;
  }

  // equality → comparison ( ( "!=" | "==" ) comparison )* ;
  Expr _equality() {
    var expr = _comparison();

    while (_match([TokenType.BANG_EQUAL, TokenType.EQUAL_EQUAL])) {
      final operator = _previous();
      final right = _comparison();
      expr = BinaryExpr(expr, operator, right);
    }

    return expr;
  }

  // comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
  Expr _comparison() {
    var expr = _term();

    final candidates = [
      TokenType.GREATER,
      TokenType.GREATER_EQUAL,
      TokenType.LESS,
      TokenType.GREATER_EQUAL
    ];

    while (_match(candidates)) {
      final operator = _previous();
      final right = _term();
      expr = BinaryExpr(expr, operator, right);
    }

    return expr;
  }

  // term → factor ( ( "-" | "+" ) factor )* ;
  Expr _term() {
    var expr = _factor();

    while (_match([TokenType.MINUS, TokenType.PLUS])) {
      final operator = _previous();
      final right = _factor();
      expr = BinaryExpr(expr, operator, right);
    }

    return expr;
  }

  // factor → unary ( ( "/" | "*" ) unary )* ;
  Expr _factor() {
    var expr = _unary();

    while (_match([TokenType.SLASH, TokenType.STAR])) {
      final operator = _previous();
      final right = _unary();
      expr = BinaryExpr(expr, operator, right);
    }

    return expr;
  }

  // unary → ( "!" | "-" ) unary | primary ;
  Expr _unary() {
    if (_match([TokenType.BANG, TokenType.MINUS])) {
      final operator = _previous();
      final right = _unary();
      final exp = UnaryExpr(operator, right);
      return exp;
    }

    final exp = _primary();
    return exp;
  }

  // primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
  Expr _primary() {
    if (_match([TokenType.FALSE])) {
      return LiteralExpr(false);
    }

    if (_match([TokenType.TRUE])) {
      return LiteralExpr(true);
    }

    if (_match([TokenType.NIL])) {
      return LiteralExpr(Nil());
    }

    if (_match([TokenType.NUMBER, TokenType.STRING])) {
      final exp = LiteralExpr(_previous().literal!);
      return exp;
    }

    if (_match([TokenType.IDENT])) {
      return VariableExpr(_previous());
    }

    if (_match([TokenType.LEFT_PAREN])) {
      final expr = _expression();
      _consume(TokenType.RIGHT_PAREN, 'Expect ) after expression');
      return GroupingExpr(expr);
    }

    throw _error(_peek(), 'Expect expression');
  }

  bool _match(List<TokenType> types) {
    for (final type in types) {
      if (_check(type)) {
        _advance();
        return true;
      }
    }

    return false;
  }

  bool _check(TokenType type) {
    if (_isAtEnd) {
      return false;
    }
    return _peek().type == type;
  }

  Token _advance() {
    if (!_isAtEnd) {
      _idx++;
    }
    return _previous();
  }

  Token _peek() => _tokens[_idx];

  Token _previous() => _tokens[_idx - 1];

  Token _consume(TokenType type, String message) {
    if (_check(type)) return _advance();
    throw _error(_peek(), message);
  }

  ParseError _error(Token token, String message) {
    return ParseError(token, message);
  }

  void _synchronize() {
    _advance();

    while (!_isAtEnd) {
      if (_previous().type == TokenType.SEMICOLON) return;

      switch (_peek().type) {
        case TokenType.CLASS:
        case TokenType.FUN:
        case TokenType.VAR:
        case TokenType.IF:
        case TokenType.WHILE:
        case TokenType.PRINT:
        case TokenType.RETURN:
          return;
        default:
          _advance();
      }
    }
  }
}
