import 'interpreter.dart';

abstract class LoxCallable {
  Object call(Interpreter interpreter, List<Object> args);
  int get arity;
}
