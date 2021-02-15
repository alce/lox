import 'token.dart';

class RuntimeError implements Exception {
  final String message;
  final Token token;

  RuntimeError(this.token, this.message);

  @override
  String toString() => '${message}\n[line ${token.line}]';
}
