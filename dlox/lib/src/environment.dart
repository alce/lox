import 'exception.dart';
import 'token.dart';

class Environment {
  final Environment? _enclosing;
  final _values = <String, Object>{};

  Environment([this._enclosing]);

  Environment? get enclosing => _enclosing;

  void define(String name, Object value) => _values[name] = value;

  Object get(Token name) {
    final value = _values[name.lexeme] ?? _enclosing?.get(name);
    return value ?? undefinedVariable(name, name.lexeme);
  }

  Object? getAt(int distance, String name) =>
      _ancestor(distance)?._values[name];

  void assign(Token name, Object value) {
    if (_values.containsKey(name.lexeme)) {
      _values[name.lexeme] = value;
      return;
    }

    if (_enclosing != null) {
      _enclosing!.assign(name, value);
      return;
    }

    undefinedVariable(name, name.lexeme);
  }

  void assignAt(int distance, Token name, Object value) =>
      _ancestor(distance)?._values[name.lexeme] = value;

  Environment? _ancestor(int distance) {
    Environment? env = this;
    for (var i = 0; i < distance; i++) {
      env = env?._enclosing;
    }
    return env;
  }

  Never undefinedVariable(Token token, String varName) {
    throw RuntimeError(token, "Undefined variable '${varName}'.");
  }
}
