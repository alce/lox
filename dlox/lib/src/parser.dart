import 'expression.dart';
import 'lox.dart';
import 'nil.dart';
import 'statement.dart';
import 'token.dart';

class _ParseError implements Exception {}

class Parser {
  final List<Token> _tokens;

  int _idx = 0;

  Parser(this._tokens);

  bool get _isAtEnd => _peek().type == TokenType.EOF;

  List<Stmt> parse() {
    final statements = <Stmt>[];

    while (!_isAtEnd) {
      final decl = _declaration();
      if (decl != null) {
        statements.add(decl);
      }
    }

    return statements;
  }

  Stmt? _declaration() {
    try {
      if (_match(TokenType.CLASS)) return _classDeclaration();
      if (_match(TokenType.FUN)) return _function('function');
      if (_match(TokenType.VAR)) return _varDeclaration();
      return _statement();
    } on _ParseError catch (_) {
      _synchronize();
    }
  }

  Stmt _classDeclaration() {
    final name = _consume(TokenType.IDENT, 'Expect class name.');

    var superclass;
    if (_match(TokenType.LESS)) {
      _consume(TokenType.IDENT, 'Expect superclass name.');
      superclass = VariableExpr(_previous());
    }

    _consume(TokenType.LEFT_BRACE, "Expect '{' before class body.");

    final methods = <FunctionStmt>[];
    while (!_check(TokenType.RIGHT_BRACE) && !_isAtEnd) {
      methods.add(_function('method'));
    }

    _consume(TokenType.RIGHT_BRACE, "Expect '}' after class body.");

    return ClassStmt(name, superclass, methods);
  }

  FunctionStmt _function(String kind) {
    final name = _consume(TokenType.IDENT, 'Expect ${kind} name.');
    _consume(TokenType.LEFT_PAREN, "Expect '(' after ${kind} name.");

    final params = <Token>[];

    if (!_check(TokenType.RIGHT_PAREN)) {
      do {
        if (params.length >= 255) {
          _error(_peek(), "Can't have more than 255 parameters.");
        }
        params.add(_consume(TokenType.IDENT, 'Expect parameter name.'));
      } while (_match(TokenType.COMMA));
    }

    _consume(TokenType.RIGHT_PAREN, "Expect ')' after parameters.");
    _consume(TokenType.LEFT_BRACE, "Expect '{' before ${kind} body.");

    return FunctionStmt(name, params, _block());
  }

  Stmt _varDeclaration() {
    final name = _consume(TokenType.IDENT, 'Expect variable name.');
    final initializer = _match(TokenType.EQUAL) ? _expression() : Nil();

    _consume(TokenType.SEMICOLON, "Expect ';' after variable declaration.");

    return VarStmt(name, initializer);
  }

  Stmt _statement() {
    if (_match(TokenType.FOR)) return _forStatement();
    if (_match(TokenType.IF)) return _ifStatement();
    if (_match(TokenType.PRINT)) return _printStatement();
    if (_match(TokenType.RETURN)) return _returnStatement();
    if (_match(TokenType.WHILE)) return _whileStatement();
    if (_match(TokenType.LEFT_BRACE)) return BlockStmt(_block());

    return _expressionStatement();
  }

  Stmt _forStatement() {
    _consume(TokenType.LEFT_PAREN, "Expect '(' after 'for'.");

    var initializer;
    if (_match(TokenType.SEMICOLON)) {
      initializer = null;
    } else if (_match(TokenType.VAR)) {
      initializer = _varDeclaration();
    } else {
      initializer = _expressionStatement();
    }

    var condition;
    if (!_check(TokenType.SEMICOLON)) {
      condition = _expression();
    }
    _consume(TokenType.SEMICOLON, "Expect ';' after loop condition.");

    var increment;
    if (!_check(TokenType.RIGHT_PAREN)) {
      increment = _expression();
    }
    _consume(TokenType.RIGHT_PAREN, "Expect ')' after for clauses.");

    var body = _statement();
    if (increment != null) {
      body = BlockStmt([body, ExpressionStmt(increment)]);
    }

    condition ??= LiteralExpr(true);
    body = WhileStmt(condition, body);

    if (initializer != null) {
      body = BlockStmt([initializer, body]);
    }

    return body;
  }

  Stmt _ifStatement() {
    _consume(TokenType.LEFT_PAREN, "Expect '(' after 'if'.");
    final condition = _expression();
    _consume(TokenType.RIGHT_PAREN, "Expect ')' after if condition.");

    final thenBranch = _statement();

    var elseBranch;
    if (_match(TokenType.ELSE)) {
      elseBranch = _statement();
    }

    return IfStmt(condition, thenBranch, elseBranch);
  }

  List<Stmt> _block() {
    final statements = <Stmt>[];

    while (!_check(TokenType.RIGHT_BRACE) && !_isAtEnd) {
      final decl = _declaration();
      if (decl != null) {
        statements.add(decl);
      }
    }

    _consume(TokenType.RIGHT_BRACE, "Expect '}' after block.");
    return statements;
  }

  Stmt _expressionStatement() {
    final expr = _expression();
    _consume(TokenType.SEMICOLON, "Expect ';' after expression.");
    return ExpressionStmt(expr);
  }

  Stmt _printStatement() {
    final value = _expression();
    _consume(TokenType.SEMICOLON, "Expect ';' after value.");
    return PrintStmt(value);
  }

  Stmt _returnStatement() {
    final keyword = _previous();
    final value = _check(TokenType.SEMICOLON) ? Nil() : _expression();
    _consume(TokenType.SEMICOLON, "Expect ';' after return value.");

    return ReturnStmt(keyword, value);
  }

  Stmt _whileStatement() {
    _consume(TokenType.LEFT_PAREN, "Expect '(' after 'while'.");
    final condition = _expression();
    _consume(TokenType.RIGHT_PAREN, "Expect ')' after condition.");
    final body = _statement();

    return WhileStmt(condition, body);
  }

  Expr _expression() => _assignment();

  Expr _assignment() {
    final expr = _or();

    if (_match(TokenType.EQUAL)) {
      final equals = _previous();
      final value = _assignment();

      if (expr is VariableExpr) {
        final name = expr.name;
        return AssignExpr(name, value);
      } else if (expr is GetExpr) {
        return SetExpr(expr.object, expr.name, value);
      }

      _error(equals, 'Invalid assignment target.');
    }

    return expr;
  }

  Expr _or() {
    var expr = _and();

    while (_match(TokenType.OR)) {
      final operator = _previous();
      final right = _and();
      expr = LogicalExpr(expr, operator, right);
    }

    return expr;
  }

  Expr _and() {
    var expr = _equality();

    while (_match(TokenType.AND)) {
      final operator = _previous();
      final right = _equality();
      expr = LogicalExpr(expr, operator, right);
    }

    return expr;
  }

  Expr _equality() {
    var expr = _comparison();

    while (_matchAny([TokenType.BANG_EQUAL, TokenType.EQUAL_EQUAL])) {
      final operator = _previous();
      final right = _comparison();
      expr = BinaryExpr(expr, operator, right);
    }

    return expr;
  }

  Expr _comparison() {
    var expr = _term();

    final candidates = [
      TokenType.GREATER,
      TokenType.GREATER_EQUAL,
      TokenType.LESS,
      TokenType.LESS_EQUAL
    ];

    while (_matchAny(candidates)) {
      final operator = _previous();
      final right = _term();
      expr = BinaryExpr(expr, operator, right);
    }

    return expr;
  }

  Expr _term() {
    var expr = _factor();

    while (_matchAny([TokenType.MINUS, TokenType.PLUS])) {
      final operator = _previous();
      final right = _factor();
      expr = BinaryExpr(expr, operator, right);
    }

    return expr;
  }

  Expr _factor() {
    var expr = _unary();

    while (_matchAny([TokenType.SLASH, TokenType.STAR])) {
      final operator = _previous();
      final right = _unary();
      expr = BinaryExpr(expr, operator, right);
    }

    return expr;
  }

  Expr _unary() {
    if (_matchAny([TokenType.BANG, TokenType.MINUS])) {
      final operator = _previous();
      final right = _unary();
      return UnaryExpr(operator, right);
    }

    return _call();
  }

  Expr _call() {
    var expr = _primary();

    while (true) {
      if (_match(TokenType.LEFT_PAREN)) {
        expr = _finishCall(expr);
      } else if (_match(TokenType.DOT)) {
        final name =
            _consume(TokenType.IDENT, "Expect property name after '.'.");
        expr = GetExpr(name, expr);
      } else {
        break;
      }
    }

    return expr;
  }

  Expr _finishCall(Expr callee) {
    final arguments = <Expr>[];

    if (!_check(TokenType.RIGHT_PAREN)) {
      do {
        if (arguments.length >= 255) {
          _error(_peek(), "Can't have more than 255 arguments.");
        }
        arguments.add(_expression());
      } while (_match(TokenType.COMMA));
    }

    final paren =
        _consume(TokenType.RIGHT_PAREN, "Expect ')' after arguments.");

    return CallExpr(callee, paren, arguments);
  }

  Expr _primary() {
    if (_match(TokenType.FALSE)) return LiteralExpr(false);
    if (_match(TokenType.TRUE)) return LiteralExpr(true);
    if (_match(TokenType.NIL)) return LiteralExpr(Nil());
    if (_match(TokenType.IDENT)) return VariableExpr(_previous());
    if (_match(TokenType.THIS)) return ThisExpr(_previous());

    if (_matchAny([TokenType.NUMBER, TokenType.STRING])) {
      return LiteralExpr(_previous().literal!);
    }

    if (_match(TokenType.SUPER)) {
      final keyword = _previous();
      _consume(TokenType.DOT, "Expect '.' after 'super'.");
      final method =
          _consume(TokenType.IDENT, 'Expect superclass method name.');
      return SuperExpr(keyword, method);
    }

    if (_match(TokenType.LEFT_PAREN)) {
      final expr = _expression();
      _consume(TokenType.RIGHT_PAREN, "Expect ')' after expression.");
      return GroupingExpr(expr);
    }

    throw _error(_peek(), 'Expect expression.');
  }

  bool _match(TokenType type) {
    if (_check(type)) {
      _advance();
      return true;
    }

    return false;
  }

  bool _matchAny(List<TokenType> types) {
    for (final type in types) {
      if (_match(type)) return true;
    }

    return false;
  }

  bool _check(TokenType type) {
    if (_isAtEnd) return false;
    return _peek().type == type;
  }

  Token _advance() {
    if (!_isAtEnd) _idx++;
    return _previous();
  }

  Token _peek() => _tokens[_idx];

  Token _previous() => _tokens[_idx - 1];

  Token _consume(TokenType type, String message) {
    if (_check(type)) return _advance();
    throw _error(_peek(), message);
  }

  _ParseError _error(Token token, String message) {
    Lox.error(token, message);
    throw _ParseError();
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
