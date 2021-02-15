import 'callable.dart';
import 'interpreter.dart';

class Clock implements LoxCallable {
  @override
  int get arity => 0;

  @override
  Object call(Interpreter interpreter, List<Object> args) =>
      DateTime.now().second;

  @override
  String toString() => '<native fn>';
}
