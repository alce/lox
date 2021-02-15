import 'package:dlox/src/environment.dart';
import 'package:dlox/src/nil.dart';

import 'callable.dart';
import 'interpreter.dart';
import 'statement.dart';

class LoxFunction implements LoxCallable {
  final FunctionStmt _declaration;

  LoxFunction(this._declaration);

  @override
  int get arity => _declaration.params.length;

  @override
  Object call(Interpreter interpreter, List<Object> args) {
    final env = Environment(interpreter.globals);

    for (var i = 0; i < _declaration.params.length; i++) {
      env.define(_declaration.params[i].lexeme, args[i]);
    }

    interpreter.executeBlock(_declaration.body, env);

    return Nil();
  }

  @override
  String toString() => '<fn ${_declaration.name.lexeme}>';
}
