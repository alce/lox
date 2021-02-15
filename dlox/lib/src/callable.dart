import 'interpreter.dart';

abstract class LoxCallable {
  int get arity;

  Object call(Interpreter interpreter, List<Object> args);
}
