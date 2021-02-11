import 'token.dart';

class LoxException implements Exception {
  final String _message;

  String get message => _message;

  LoxException(this._message);
}

class SyntaxError extends LoxException {
  SyntaxError(String message) : super(message);
}

class RuntimeError extends LoxException {
  final Token _token;

  RuntimeError(this._token, String message) : super(message);

  @override
  String toString() => 'RuntimeError: ${message}';
}
