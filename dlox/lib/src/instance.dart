import 'class.dart';
import 'exception.dart';
import 'token.dart';

class LoxInstance {
  final LoxClass _klass;
  final _fields = <String, Object>{};

  LoxInstance(this._klass);

  Object get(Token name) {
    final val =
        _fields[name.lexeme] ?? _klass.findMethod(name.lexeme)?.bind(this);

    if (val != null) return val;

    throw RuntimeError(name, "Undefined property '${name.lexeme}'.");
  }

  void set(Token name, Object value) => _fields[name.lexeme] = value;

  @override
  String toString() => '${_klass.name} instance';
}
