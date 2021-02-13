import 'src/interpreter.dart';
import 'src/parser.dart';
import 'src/scanner.dart';

export 'src/exception.dart';

void run(String source, Interpreter interpreter) {
  final scanner = Scanner(source);
  final parser = Parser(scanner.scan());

  interpreter.interpret(parser.parse());
}
