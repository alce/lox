import 'callable.dart';
import 'environment.dart';
import 'interpreter.dart';
import 'nil.dart';
import 'return.dart';
import 'statement.dart';

class LoxFunction implements LoxCallable {
  final FunctionStmt _declaration;
  final Environment _closure;

  LoxFunction(this._declaration, this._closure);

  @override
  int get arity => _declaration.params.length;

  @override
  Object call(Interpreter interpreter, List<Object> args) {
    final env = Environment(_closure);

    for (var i = 0; i < _declaration.params.length; i++) {
      env.define(_declaration.params[i].lexeme, args[i]);
    }

    try {
      interpreter.executeBlock(_declaration.body, env);
    } on Return catch (r) {
      return r.value;
    }

    return Nil();
  }

  @override
  String toString() => '<fn ${_declaration.name.lexeme}>';
}
