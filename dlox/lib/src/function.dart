import 'callable.dart';
import 'environment.dart';
import 'instance.dart';
import 'interpreter.dart';
import 'nil.dart';
import 'return.dart';
import 'statement.dart';

class LoxFunction implements LoxCallable {
  final FunctionStmt _declaration;
  final Environment _closure;
  final bool _isInitializer;

  LoxFunction(this._declaration, this._closure, this._isInitializer);

  @override
  int get arity => _declaration.params.length;

  @override
  Object call(Interpreter interpreter, List<Object> args) {
    final env = Environment(_closure);

    _declaration.params
        .asMap()
        .forEach((i, v) => env.define(v.lexeme, args[i]));

    try {
      interpreter.executeBlock(_declaration.body, env);
    } on Return catch (r) {
      return (_isInitializer ? _closure.getAt(0, 'this') : r.value) ?? Nil();
    }

    if (_isInitializer) return _closure.getAt(0, 'this') ?? Nil();

    return Nil();
  }

  LoxFunction bind(LoxInstance instance) {
    final env = Environment(_closure);
    env.define('this', instance);
    return LoxFunction(_declaration, env, _isInitializer);
  }

  @override
  String toString() => '<fn ${_declaration.name.lexeme}>';
}
