import 'src/interpreter.dart';
import 'src/parser.dart';
import 'src/scanner.dart';

export 'src/exception.dart';

void run(String source) {
  final interpreter = Interpreter();

  final scanner = Scanner(source);
  final parser = Parser(scanner.scan());
  final expr = parser.parse();

  if (expr != null) {
    interpreter.interpret(expr);
  } else {
    print('Some error happened. Call someone.');
  }
}
