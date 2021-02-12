import 'src/interpreter.dart';
import 'src/parser.dart';
import 'src/scanner.dart';

export 'src/exception.dart';

void run(String source) {
  final interpreter = Interpreter();
  final scanner = Scanner(source);
  final parser = Parser(scanner.scan());
  final statements = parser.parse();

  interpreter.interpret(statements);
}
