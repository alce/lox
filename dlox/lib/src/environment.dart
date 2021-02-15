import 'exception.dart';
import 'token.dart';

class Environment {
  final Environment? _enclosing;
  final _values = <String, Object>{};

  Environment([this._enclosing]);

  void define(String name, Object value) => _values[name] = value;

  Object get(Token name) {
    if (_values.containsKey(name.lexeme)) {
      return _values[name.lexeme]!;
    }

    if (_enclosing != null) {
      return _enclosing!.get(name);
    }

    throw RuntimeError(name, "Undefined variable '${name.lexeme}'.");
  }

  void assign(Token name, Object value) {
    if (_values.containsKey(name.lexeme)) {
      _values[name.lexeme] = value;
      return;
    }

    if (_enclosing != null) {
      _enclosing!.assign(name, value);
      return;
    }

    throw RuntimeError(name, "Undefined variable '${name.lexeme}'.");
  }
}
