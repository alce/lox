import 'package:dlox/src/ast_printer.dart';

import 'src/parser.dart';
import 'src/scanner.dart';

void run(String source) {
  final scanner = Scanner(source);
  final parser = Parser(scanner.scan());
  final expr = parser.parse();

  if (expr != null) {
    print(AstPrinter().print(expr));
  } else {
    print('Some error happened. Call someone.');
  }
}
