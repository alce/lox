import 'expression.dart';
import 'token.dart';

class _ParseError implements Exception {}

class Parser {
  final List<Token> _tokens;

  int _idx = 0;

  Parser(this._tokens);

  bool get _isAtEnd => _peek().type == TokenType.EOF;

  Expression? parse() {
    try {
      return _expression();
    } on _ParseError catch (_) {
      return null;
    }
  }

  // expression → equality ;
  Expression _expression() => _equality();

  // equality → comparison ( ( "!=" | "==" ) comparison )* ;
  Expression _equality() {
    var expr = _comparison();

    while (_match([TokenType.BANG_EQUAL, TokenType.EQUAL_EQUAL])) {
      final operator = _previous();
      final right = _comparison();
      expr = BinaryExpression(expr, operator, right);
    }

    return expr;
  }

  // comparison → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
  Expression _comparison() {
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
      expr = BinaryExpression(expr, operator, right);
    }

    return expr;
  }

  // term → factor ( ( "-" | "+" ) factor )* ;
  Expression _term() {
    var expr = _factor();

    while (_match([TokenType.MINUS, TokenType.PLUS])) {
      final operator = _previous();
      final right = _factor();
      expr = BinaryExpression(expr, operator, right);
    }

    return expr;
  }

  // factor → unary ( ( "/" | "*" ) unary )* ;
  Expression _factor() {
    var expr = _unary();

    while (_match([TokenType.SLASH, TokenType.STAR])) {
      final operator = _previous();
      final right = _unary();
      expr = BinaryExpression(expr, operator, right);
    }

    return expr;
  }

  // unary → ( "!" | "-" ) unary | primary ;
  Expression _unary() {
    if (_match([TokenType.BANG, TokenType.MINUS])) {
      final operator = _previous();
      final right = _unary();
      return UnaryExpression(operator, right);
    }

    return _primary();
  }

  // primary → NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
  Expression _primary() {
    if (_match([TokenType.FALSE])) return LiteralExpression(false);
    if (_match([TokenType.TRUE])) return LiteralExpression(true);
    if (_match([TokenType.NIL])) return LiteralExpression(null);

    if (_match([TokenType.NUMBER, TokenType.STRING])) {
      return LiteralExpression(_previous().literal);
    }

    if (_match([TokenType.LEFT_PAREN])) {
      final expr = _expression();
      _consume(TokenType.RIGHT_PAREN, 'Expect ) after expression');
      return GroupingExpression(expr);
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

  _ParseError _error(Token token, String message) {
    // Lox.error(token, message);
    return _ParseError();
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