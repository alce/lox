import 'package:dlox/lox.dart';

import 'token.dart';

class Environment {
  final _values = <String, Object>{};

  // implies variables can be redefined.
  void define(String name, Object value) => _values[name] = value;

  Object get(Token name) {
    final ret = _values[name.lexeme];

    if (ret == null) {
      throw RuntimeError(name, 'Undefined variable ${name.lexeme}.');
    }

    return ret;
  }

  // no implicit variable declaration.
  // a = 3; => throws
  // var a;
  // a = 3 => ok
  void assign(Token name, Object value) {
    if (_values.containsKey(name.lexeme)) {
      _values[name.lexeme] = value;
    } else {
      throw RuntimeError(name, 'Undefined variable ${name.lexeme}.');
    }
  }
}
