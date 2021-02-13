import 'token.dart';

class LoxException implements Exception {
  final String _message;
  final Token _token;

  String get message => _message;
  Token get token => _token;

  LoxException(this._token, this._message);
}

class SyntaxError extends LoxException {
  SyntaxError(Token token, String message) : super(token, message);

  @override
  String toString() {
    final location = token.type == TokenType.EOF ? 'end' : token.lexeme;
    return '[line ${token.line}] Error at ${location}: ${message}';
  }
}

class RuntimeError extends LoxException {
  RuntimeError(Token token, String message) : super(token, message);

  @override
  String toString() => '${message}\n [line ${token.line}]';
}
