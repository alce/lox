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
}
