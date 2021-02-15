import 'callable.dart';
import 'instance.dart';
import 'interpreter.dart';

class LoxClass implements LoxCallable {
  final String name;

  LoxClass(this.name);

  @override
  int get arity => 0;

  @override
  Object call(Interpreter interpreter, List<Object> args) {
    final instance = LoxInstance(this);
    return instance;
  }

  @override
  String toString() => name;
}
