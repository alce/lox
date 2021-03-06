import 'callable.dart';
import 'function.dart';
import 'instance.dart';
import 'interpreter.dart';

class LoxClass implements LoxCallable {
  final String name;
  final Map<String, LoxFunction> _methods;
  final LoxClass? superclass;

  LoxClass(this.name, this.superclass, this._methods);

  @override
  int get arity => findMethod('init')?.arity ?? 0;

  @override
  Object call(Interpreter interpreter, List<Object> args) {
    final instance = LoxInstance(this);
    findMethod('init')?.bind(instance).call(interpreter, args);
    return instance;
  }

  LoxFunction? findMethod(String name) =>
      _methods[name] ?? superclass?.findMethod(name);

  @override
  String toString() => name;
}
