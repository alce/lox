import 'token.dart';

class LoxException implements Exception {
  final String _message;
  final Token? _token;

  String get message => _message;
  Token? get token => _token;

  LoxException(this._message, [this._token]);
}

class SyntaxError extends LoxException {
  final int line;

  SyntaxError(this.line, String message) : super(message);

  @override
  String toString() {
    return '[line ${line}] Error: ${message}';
  }
}

class RuntimeError extends LoxException {
  RuntimeError(Token token, String message) : super(message, token);

  @override
  String toString() => '${message}\n[line ${token!.line}]';
}

class ParseError extends LoxException {
  ParseError(Token token, String message) : super(message, token);

  @override
  String toString() {
    final location = token!.type == TokenType.EOF ? 'end' : token!.lexeme;
    return '[line ${token!.line}] Error at ${location}: ${message}';
  }
}
