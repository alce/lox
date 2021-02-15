import 'callable.dart';
import 'function.dart';
import 'instance.dart';
import 'interpreter.dart';

class LoxClass implements LoxCallable {
  final String name;
  final Map<String, LoxFunction> _methods;

  LoxClass(this.name, this._methods);

  @override
  int get arity => 0;

  @override
  Object call(Interpreter interpreter, List<Object> args) {
    final instance = LoxInstance(this);
    return instance;
  }

  LoxFunction? findMethod(String name) => _methods[name];

  @override
  String toString() => name;
}
