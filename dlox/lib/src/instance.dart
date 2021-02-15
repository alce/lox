import 'class.dart';
import 'exception.dart';
import 'token.dart';

class LoxInstance {
  final LoxClass _klass;
  final _fields = <String, Object>{};

  LoxInstance(this._klass);

  Object get(Token name) {
    final obj = _fields[name.lexeme];

    if (obj != null) {
      return obj;
    }

    final method = _klass.findMethod(name.lexeme);

    if (method != null) {
      return method.bind(this);
    }

    throw RuntimeError(name, "Undefined property '${name.lexeme}'.");
  }

  void set(Token name, Object value) => _fields[name.lexeme] = value;

  @override
  String toString() => '${_klass.name} instance';
}
